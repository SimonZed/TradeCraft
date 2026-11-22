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

/// The renderer configuration. `WGPU_BACKEND`, when set, always takes priority.
pub(crate) fn wgpu_options(choice: GraphicsBackend) -> WgpuConfiguration {
    let mut options = WgpuConfiguration::default();
    if let Some(backends) = requested_backends(Backends::from_env(), choice)
        && let WgpuSetup::CreateNew(setup) = &mut options.wgpu_setup
    {
        setup.instance_descriptor.backends = backends;
    }
    options
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
}
