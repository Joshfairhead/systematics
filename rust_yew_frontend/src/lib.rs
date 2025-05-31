use yew::{html, Component, Context, Html, Callback};
use wasm_bindgen::prelude::*;

mod components; // Declare the components module
use components::system_selector::SystemSelector; // Import the SystemSelector

pub struct App {
    selected_system_num: i32,
}

pub enum Msg {
    SystemSelected(i32),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_system_num: 1,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SystemSelected(system_num) => {
                self.selected_system_num = system_num;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_system_selected = ctx.link().callback(Msg::SystemSelected);
        
        // Determine the image URL based on the selected system
        let image_url = match self.selected_system_num {
            1 => "monad.png",
            2 => "dyad.png",
            3 => "triad.png",
            4 => "tetrad.png",
            5 => "pentad.png",
            6 => "hexad.png",
            7 => "heptad.png",
            8 => "octad.png",
            _ => "default.png",
        };

        // Create system-specific CSS class
        let system_class = format!("system-{}", self.selected_system_num);

        html! {
            <div class="app-container">
                <div class="system-selector-container">
                    <SystemSelector {on_system_selected} />
                </div>
                <div class={format!("main-content {}", system_class)}>
                    <img src={image_url} alt={format!("System {}", self.selected_system_num)} />
                </div>
            </div>
        }
    }
}

// When building a library, the main function is not needed here.
// Trunk will handle the initialization. 
// However, Yew expects a `main` function or for the app to be mounted.
// For a library, we typically expose a function that can be called from JS to start the app.
// Or, ensure the `yew::Renderer::<App>::new().render();` call is made, 
// but it needs to be linked correctly for Wasm.
// For Trunk, it usually finds the main function or relies on wasm-bindgen to expose it.

// Let's adjust this for a common library pattern with wasm-bindgen.
// We need a public function that JavaScript can call to start the app.

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
} 