use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<AndroidTvCheck<R>> {
    Ok(AndroidTvCheck(app.clone()))
}

/// Access to the android-tv-check APIs.
pub struct AndroidTvCheck<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AndroidTvCheck<R> {
    pub fn check(&self) -> crate::Result<CheckResponse> {
        Ok(CheckResponse {
            is_android_tv: true,
        })
    }
}
