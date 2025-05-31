use yew::prelude::*;
use wasm_bindgen::prelude::*;

mod components; // Declare the components module
use components::system_selector::SystemSelector; // Import the SystemSelector

#[function_component(App)]
fn app() -> Html {
    // State to hold the currently selected system number. Default to 1 (Monad).
    let selected_system_num = use_state(|| 1_i32);

    let on_system_change = {
        let selected_system_num = selected_system_num.clone();
        Callback::from(move |new_system_num: i32| {
            selected_system_num.set(new_system_num);
        })
    };

    // Determine image source based on selection
    let (image_src, image_alt) = match *selected_system_num {
        1 => ("monad.png", "Monad System"),
        2 => ("dyad.png", "Dyad System"),
        3 => ("triad.png", "Triad System"),
        4 => ("tetrad.png", "Tetrad System"),
        5 => ("pentad.png", "Pentad System"),
        6 => ("hexad.png", "Hexad System"),
        7 => ("heptad.png", "Heptad System"),
        8 => ("octad.png", "Octad System"),
        _ => ("default.png", "System Diagram"),
    };

    html! {
        <div class="app-container">
            <SystemSelector on_system_change={on_system_change} />
            <div class="main-content">
                <img 
                    src={image_src}
                    alt={image_alt}
                    style="max-width: 500px; max-height: 500px;"
                />
            </div>
        </div>
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