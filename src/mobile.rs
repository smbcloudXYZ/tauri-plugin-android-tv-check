use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_android_tv_check);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<AndroidTvCheck<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("", "AndroidTvCheckPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_android_tv_check)?;
    Ok(AndroidTvCheck(handle))
}

/// Access to the android-tv-check APIs.
pub struct AndroidTvCheck<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AndroidTvCheck<R> {
    pub fn check(&self) -> crate::Result<CheckResponse> {
        self.0.run_mobile_plugin("check", ()).map_err(Into::into)
    }
}
