pub mod benchmark;
pub mod code_size;
pub mod codestorage;
pub mod documentation;
pub mod editor_window;
pub mod filesystem;
pub mod format;
pub mod game;
pub mod js;
pub mod leaderboard;
pub mod simulation_window;
pub mod telemetry;
pub mod toolbar;
pub mod ui;
pub mod userid;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub fn version() -> String {
    let mut fragments = vec![built_info::GIT_VERSION.unwrap_or("unknown")];
    if built_info::GIT_DIRTY == Some(true) {
        fragments.push("dirty");
    }
    fragments.join("-")
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/scenario/:name")]
    Scenario { name: String },
    #[at("/demo/:name")]
    Demo { name: String },
    #[at("/benchmark/:name")]
    Benchmark { name: String },
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => switch(&Route::Scenario {
            name: "welcome".to_owned(),
        }),
        Route::Scenario { name } => html! {
            <game::Game scenario={name.clone()} demo=false version={version()} />
        },
        Route::Demo { name } => html! {
            <game::Game scenario={name.clone()} demo=true version={version()} />
        },
        Route::Benchmark { name } => html! {
            <benchmark::Benchmark scenario={name.clone()} />
        },
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    log::info!("Version {}", &version());
    let userid = userid::get_userid();
    log::info!("userid {}", &userid);
    log::info!("username {}", &userid::get_username());
    js::golden_layout::init();
    yew::start_app_in_element::<Main>(
        gloo_utils::document()
            .get_element_by_id("yew")
            .expect("a #yew element"),
    );
    Ok(())
}
