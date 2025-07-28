use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::AndroidTvCheckExt;
use crate::Result;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.android_tv_check().ping(payload)
}

#[command]
pub(crate) async fn is_android_tv<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    app.android_tv_check().check()
}
