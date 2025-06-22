use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_environment: String,
    pub switching_environment: bool,
    pub on_switch_testing: Callback<()>,
    pub on_switch_development: Callback<()>,
}

#[function_component(EnvironmentSwitcher)]
pub fn environment_switcher(props: &Props) -> Html {
    let is_autopoietic = props.current_environment == "development";
    
    let switch_to_testing = {
        let callback = props.on_switch_testing.clone();
        Callback::from(move |_| callback.emit(()))
    };
    
    let switch_to_development = {
        let callback = props.on_switch_development.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <div class="environment-controls">
            <div class="environment-switch">
                <button 
                    class={if props.current_environment == "testing" { "env-button active" } else { "env-button" }}
                    onclick={switch_to_testing}
                    disabled={props.switching_environment}
                >
                    {"Testing"}
                </button>
                <button 
                    class={if props.current_environment == "development" { "env-button active autopoietic" } else { "env-button" }}
                    onclick={switch_to_development}
                    disabled={props.switching_environment}
                >
                    {"Autopoietic"}
                </button>
            </div>
        </div>
    }
} 