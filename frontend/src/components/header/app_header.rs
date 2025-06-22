use yew::prelude::*;
use super::EnvironmentSwitcher;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_environment: String,
    pub switching_environment: bool,
    pub on_switch_testing: Callback<()>,
    pub on_switch_development: Callback<()>,
}

#[function_component(AppHeader)]
pub fn app_header(props: &Props) -> Html {
    let is_autopoietic = props.current_environment == "development";
    let header_class = if is_autopoietic { "app-header autopoietic" } else { "app-header" };

    html! {
        <header class={header_class}>
            <div class="header-content">
                <div class="header-title">
                    <h1>{"SysteMaster"}</h1>
                    <p>{"Systematic Input Interface"}</p>
                </div>
                <EnvironmentSwitcher
                    current_environment={props.current_environment.clone()}
                    switching_environment={props.switching_environment}
                    on_switch_testing={props.on_switch_testing.clone()}
                    on_switch_development={props.on_switch_development.clone()}
                />
            </div>
        </header>
    }
} 