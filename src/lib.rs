mod api;

use api::Ctx;
use tauri::{plugin::TauriPlugin, Runtime};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let router = api::new().build().arced();
    rspc::integrations::tauri::plugin(router, || Ctx {})
}
