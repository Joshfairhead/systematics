use yew::{html, Component, Context, Html, Properties, Callback, classes};

// Define the properties for the SystemSelector component
#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_system_selected: Callback<i32>,
    pub selected_system: i32,
    #[prop_or(false)]
    pub disabled: bool,
}

pub struct SystemSelector;

pub enum Msg {
    SystemSelected(i32),
}

impl Component for SystemSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SystemSelected(system_num) => {
                ctx.props().on_system_selected.emit(system_num);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected_system = ctx.props().selected_system;
        let disabled = ctx.props().disabled;

        // System definitions with short names for the buttons
        let systems = [
            (1, "Monad"),
            (2, "Dyad"), 
            (3, "Triad"),
            (4, "Tetrad"),
            (5, "Pentad"),
            (6, "Hexad"),
            (7, "Heptad"),
            (8, "Octad"),
        ];

        html! {
            <div class="system-selector-tabs">
                {for systems.iter().map(|(num, name)| {
                    let system_num = *num;
                    let is_active = selected_system == system_num;
                    let onclick = ctx.link().callback(move |_| Msg::SystemSelected(system_num));
                    
                    html! {
                        <button 
                            class={classes!(
                                "system-tab-button", 
                                if is_active { "active" } else { "" }
                            )}
                            {onclick}
                            {disabled}
                        >
                            {name}
                        </button>
                    }
                })}
            </div>
        }
    }
} 