//! Graphics backend selection for the wgpu renderer.

use eframe::egui_wgpu::{WgpuConfiguration, WgpuSetup};
use eframe::wgpu::Backends;

/// The user's graphics backend choice, applied at the next start.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum GraphicsBackend {
    #[default]
    Auto,
    Dx12,
    Vulkan,
    OpenGl,
}

impl GraphicsBackend {
    /// Choices offered on this platform.
    pub(crate) const AVAILABLE: &'static [Self] = &[
        Self::Auto,
        #[cfg(windows)]
        Self::Dx12,
        #[cfg(not(target_os = "macos"))]
        Self::Vulkan,
        Self::OpenGl,
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Dx12 => "DX12",
            Self::Vulkan => "Vulkan",
            Self::OpenGl => "OpenGL",
        }
    }

    fn key(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Dx12 => "dx12",
            Self::Vulkan => "vulkan",
            Self::OpenGl => "opengl",
        }
    }

    fn from_key(key: &str) -> Self {
        Self::AVAILABLE
            .iter()
            .copied()
            .find(|choice| choice.key() == key)
            .unwrap_or_default()
    }

    /// Backends to request. `None` keeps wgpu's own default.
    ///
    /// Auto selects DX12 on Windows: some Intel iGPU Vulkan drivers crash during
    /// initialisation. Other platforms keep wgpu's defaults.
    fn backends(self) -> Option<Backends> {
        match self {
            Self::Auto => cfg!(windows).then_some(Backends::DX12),
            Self::Dx12 => Some(Backends::DX12),
            Self::Vulkan => Some(Backends::VULKAN),
            Self::OpenGl => Some(Backends::GL),
        }
    }

    fn path() -> Option<std::path::PathBuf> {
        Some(tradecraft_providers::settings::data_dir()?.join(
            if cfg!(feature = "vibestrader-credential") {
                "graphics-personal.json"
            } else {
                "graphics-community.json"
            },
        ))
    }

    pub(crate) fn load() -> Self {
        Self::path()
            .and_then(|path| std::fs::read(path).ok())
            .and_then(|data| serde_json::from_slice::<serde_json::Value>(&data).ok())
            .and_then(|value| value["backend"].as_str().map(Self::from_key))
            .unwrap_or_default()
    }

    pub(crate) fn save(self) -> Result<(), String> {
        let path = Self::path().ok_or("Settings folder is unavailable")?;
        let parent = path.parent().ok_or("Settings folder is unavailable")?;
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Could not save graphics backend: {e}"))?;
        let data = serde_json::json!({"backend": self.key()});
        crate::terminal_storage::write_atomic(&path, &data.to_string())
            .map_err(|e| format!("Could not save graphics backend: {e}"))
    }
}

/// Backends to request: `WGPU_BACKEND` first, then the saved choice.
fn requested_backends(env: Option<Backends>, choice: GraphicsBackend) -> Option<Backends> {
    if env.is_some() {
        return None;
    }
    choice.backends()
}

fn backend_key(backends: Option<Backends>) -> &'static str {
    match backends {
        Some(b) if b == Backends::DX12 => "dx12",
        Some(b) if b == Backends::VULKAN => "vulkan",
        Some(b) if b == Backends::GL => "opengl",
        _ => "default",
    }
}

/// After a start that never finished initialising graphics, try DX12 and then OpenGL.
/// Windows only; `None` means no fallback applies.
fn fallback_after(previous_attempt: &str) -> Option<Backends> {
    if !cfg!(windows) {
        return None;
    }
    match previous_attempt {
        "dx12" => Some(Backends::GL),
        "opengl" => None,
        _ => Some(Backends::DX12),
    }
}

/// The backends this start will request, and whether a crash fallback chose them.
fn plan(
    env: Option<Backends>,
    choice: GraphicsBackend,
    previous_attempt: Option<&str>,
) -> (Option<Backends>, bool) {
    if env.is_some() {
        return (None, false);
    }
    match previous_attempt.and_then(fallback_after) {
        Some(backends) => (Some(backends), true),
        None => (requested_backends(env, choice), false),
    }
}

fn marker_path() -> Option<std::path::PathBuf> {
    Some(tradecraft_providers::settings::data_dir()?.join("graphics-init.marker"))
}

/// The renderer configuration, with the backend request for the log.
///
/// A marker file is written before graphics initialise and removed once a few frames
/// have rendered. If it is still there at the next start, the previous start crashed
/// during graphics initialisation, so a safer backend is tried.
pub(crate) fn startup(choice: GraphicsBackend) -> WgpuConfiguration {
    let env = Backends::from_env();
    let previous = marker_path()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .map(|text| text.trim().to_owned());
    if let Some(previous) = &previous {
        crate::startup_log::log(format!(
            "previous start did not finish initialising graphics (backend {previous})"
        ));
    }
    let (backends, fell_back) = plan(env, choice, previous.as_deref());
    if fell_back {
        crate::startup_log::log("automatic graphics fallback for this start only");
    }
    crate::startup_log::log(format!(
        "graphics backend requested: setting={}, WGPU_BACKEND={}, effective={}",
        choice.label(),
        std::env::var("WGPU_BACKEND").unwrap_or_else(|_| "unset".into()),
        match (env, backends) {
            (Some(env), _) => format!("{env:?}"),
            (None, Some(backends)) => format!("{backends:?}"),
            (None, None) => "wgpu default".into(),
        }
    ));
    if env.is_none()
        && let Some(path) = marker_path()
    {
        let _ = std::fs::write(path, backend_key(backends));
    }
    let mut options = WgpuConfiguration::default();
    if let Some(backends) = backends
        && let WgpuSetup::CreateNew(setup) = &mut options.wgpu_setup
    {
        setup.instance_descriptor.backends = backends;
    }
    options
}

static FRAMES: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// Call once per frame; clears the crash marker after the second frame.
pub(crate) fn frame_finished() {
    use std::sync::atomic::Ordering;
    if FRAMES.load(Ordering::Relaxed) >= 2 {
        return;
    }
    if FRAMES.fetch_add(1, Ordering::Relaxed) == 1 {
        if let Some(path) = marker_path() {
            let _ = std::fs::remove_file(path);
        }
        crate::startup_log::log("graphics initialised: first frames rendered");
    }
}

/// Logs the adapter chosen by wgpu.
pub(crate) fn log_adapter(state: Option<&eframe::egui_wgpu::RenderState>) {
    let Some(state) = state else {
        crate::startup_log::log("graphics adapter: unavailable (no wgpu render state)");
        return;
    };
    let info = state.adapter.get_info();
    crate::startup_log::log(format!(
        "graphics adapter: {} | backend {:?} | type {:?} | driver {} {}",
        info.name, info.backend, info.device_type, info.driver, info.driver_info
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_variable_overrides_setting_and_default() {
        assert_eq!(
            requested_backends(Some(Backends::VULKAN), GraphicsBackend::OpenGl),
            None
        );
        assert_eq!(
            requested_backends(None, GraphicsBackend::Auto),
            cfg!(windows).then_some(Backends::DX12)
        );
        assert_eq!(
            requested_backends(None, GraphicsBackend::Vulkan),
            Some(Backends::VULKAN)
        );
        assert_eq!(
            requested_backends(None, GraphicsBackend::OpenGl),
            Some(Backends::GL)
        );
    }

    #[test]
    fn saved_names_round_trip_and_unknown_names_fall_back_to_auto() {
        for choice in GraphicsBackend::AVAILABLE {
            assert_eq!(GraphicsBackend::from_key(choice.key()), *choice);
        }
        assert_eq!(GraphicsBackend::from_key("nonsense"), GraphicsBackend::Auto);
    }

    #[test]
    fn crash_fallback_tries_dx12_then_opengl_on_windows() {
        let auto = GraphicsBackend::Auto;
        assert_eq!(plan(None, auto, None), (auto.backends(), false));
        if cfg!(windows) {
            assert_eq!(
                plan(None, auto, Some("vulkan")),
                (Some(Backends::DX12), true)
            );
            assert_eq!(plan(None, auto, Some("dx12")), (Some(Backends::GL), true));
            assert_eq!(plan(None, auto, Some("opengl")), (auto.backends(), false));
        } else {
            assert_eq!(plan(None, auto, Some("dx12")), (auto.backends(), false));
        }
        assert_eq!(
            plan(Some(Backends::VULKAN), auto, Some("dx12")),
            (None, false)
        );
    }
}
