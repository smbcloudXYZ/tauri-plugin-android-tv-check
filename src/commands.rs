use {
    crate::models::CheckResponse,
    crate::AndroidTvCheckExt,
    crate::Result,
    tauri::{command, AppHandle, Runtime},
};

#[command]
pub(crate) async fn check<R: Runtime>(app: AppHandle<R>) -> Result<CheckResponse> {
    app.android_tv_check().check()
}
