use crate::leaderboard::Leaderboard;
use yew::{create_portal, function_component, html, Html, Properties};

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub host: web_sys::Element,
    pub scenario_name: String,
}

#[function_component]
pub fn LeaderboardWindow(props: &Props) -> Html {
    create_portal(
        html! {
            <div class="leaderboard">
                {
                    if props.scenario_name == "welcome" {
                        html! { <p>{ "Choose a scenario from the input at the top-right of the page." }</p> }
                    } else {
                        html! { <Leaderboard scenario_name={props.scenario_name.clone()} /> }
                    }
                }
            </div>
        },
        props.host.clone(),
    )
}
