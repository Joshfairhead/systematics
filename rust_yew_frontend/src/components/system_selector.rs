use yew::{html, Component, Context, Html, Properties, Callback};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

// Define the properties for the SystemSelector component (if any needed later)
#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_system_selected: Callback<i32>,
    pub selected_system: i32,
}

pub struct SystemSelector;

pub enum Msg {
    SystemChanged(String),
}

impl Component for SystemSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SystemChanged(value) => {
                if let Ok(system_num) = value.parse::<i32>() {
                    ctx.props().on_system_selected.emit(system_num);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(|e: yew::events::Event| {
            let target = e.target().unwrap();
            let select = target.dyn_into::<HtmlSelectElement>().unwrap();
            Msg::SystemChanged(select.value())
        });

        let selected_system = ctx.props().selected_system;

        html! {
            <div>
                <label for="system-selector">{"Select System: "}</label>
                <select id="system-selector" {onchange}>
                    <option value="1" selected={selected_system == 1}>{"Monad (1)"}</option>
                    <option value="2" selected={selected_system == 2}>{"Dyad (2)"}</option>
                    <option value="3" selected={selected_system == 3}>{"Triad (3)"}</option>
                    <option value="4" selected={selected_system == 4}>{"Tetrad (4)"}</option>
                    <option value="5" selected={selected_system == 5}>{"Pentad (5)"}</option>
                    <option value="6" selected={selected_system == 6}>{"Hexad (6)"}</option>
                    <option value="7" selected={selected_system == 7}>{"Heptad (7)"}</option>
                    <option value="8" selected={selected_system == 8}>{"Octad (8)"}</option>
                    // <option value="9">{"Ennead (9)"}</option>
                    // <option value="10">{"Decad (10)"}</option>
                    // <option value="11">{"Undecad (11)"}</option>
                    // <option value="12">{"Dodecad (12)"}</option>
                </select>
            </div>
        }
    }
} 