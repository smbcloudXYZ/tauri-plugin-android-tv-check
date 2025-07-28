use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::AndroidTvCheck;
#[cfg(mobile)]
use mobile::AndroidTvCheck;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the android-tv-check APIs.
pub trait AndroidTvCheckExt<R: Runtime> {
    fn android_tv_check(&self) -> &AndroidTvCheck<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AndroidTvCheckExt<R> for T {
    fn android_tv_check(&self) -> &AndroidTvCheck<R> {
        self.state::<AndroidTvCheck<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("android-tv-check")
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::is_android_tv
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let android_tv_check = mobile::init(app, api)?;
            #[cfg(desktop)]
            let android_tv_check = desktop::init(app, api)?;
            app.manage(android_tv_check);
            Ok(())
        })
        .build()
}
