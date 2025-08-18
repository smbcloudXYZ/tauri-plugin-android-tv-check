use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.androidTvCheck";

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<AndroidTvCheck<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "AndroidTvCheckPlugin")?;
    Ok(AndroidTvCheck(handle))
}

/// Access to the android-tv-check APIs.
pub struct AndroidTvCheck<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AndroidTvCheck<R> {
    pub fn check(&self) -> crate::Result<CheckResponse> {
        match self.0.run_mobile_plugin("check", ()) {
            Ok(res) => {
                println!("tauri-plugin-android-tv-check: {:?}", res);
                Ok(res)
            }
            Err(err) => Err(err.into()),
        }
    }
}
