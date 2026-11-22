//! Graphics backend selection for the wgpu renderer.

use eframe::egui_wgpu::{WgpuConfiguration, WgpuSetup};
use eframe::wgpu::Backends;

/// Backends to request when `WGPU_BACKEND` is not set. `None` keeps wgpu's own default.
///
/// Windows defaults to DX12: some Intel iGPU Vulkan drivers crash during initialisation.
fn default_backends(env: Option<Backends>) -> Option<Backends> {
    if env.is_some() {
        return None;
    }
    cfg!(windows).then_some(Backends::DX12)
}

/// The renderer configuration. `WGPU_BACKEND`, when set, always takes priority.
pub(crate) fn wgpu_options() -> WgpuConfiguration {
    let mut options = WgpuConfiguration::default();
    if let Some(backends) = default_backends(Backends::from_env())
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
    fn environment_variable_overrides_the_platform_default() {
        assert_eq!(default_backends(Some(Backends::VULKAN)), None);
        assert_eq!(
            default_backends(None),
            cfg!(windows).then_some(Backends::DX12)
        );
    }
}
