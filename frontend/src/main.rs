#![allow(clippy::redundant_closure_call)]
#![allow(clippy::await_holding_refcell_ref)]
#![allow(clippy::drop_non_drop)]
#![allow(non_snake_case)]

pub mod util;

pub mod app;
pub mod components;
pub mod page;

use cfg_if::cfg_if;
use util::ApiClient;

pub const ROOT_API_URL: &str = uchat_endpoint::app_url::API_URL;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

fn main() {
    init_log();
    ApiClient::init();
    dioxus_web::launch(app::App)
}

// NOTE Creating a prelude module for modules that are used frequently
mod prelude {
    pub use crate::page;

    pub use crate::util::api_client::fetch_json;
    pub use crate::util::{async_handler, maybe_class, ret_if, sync_handler, toasty, ApiClient};

    pub use crate::components::app_bar::{self, AppBar, AppBarImgButton};
    pub use crate::components::local_profile::{use_local_profile, LocalProfile};
    pub use crate::components::post::PublicPostEntry;
    pub use crate::components::toaster::use_toaster;

    pub use dioxus_router::{use_route, use_router};
}
