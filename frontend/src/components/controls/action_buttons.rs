use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub creation_mode: bool,
    pub saving: bool,
    pub on_load: Callback<()>,
    pub on_create: Callback<()>,
    pub on_save: Callback<()>,
    pub on_cancel: Callback<()>,
}

#[function_component(ActionButtons)]
pub fn action_buttons(props: &Props) -> Html {
    let load_callback = {
        let callback = props.on_load.clone();
        Callback::from(move |_| callback.emit(()))
    };
    
    let create_callback = {
        let callback = props.on_create.clone();
        Callback::from(move |_| callback.emit(()))
    };
    
    let save_callback = {
        let callback = props.on_save.clone();
        Callback::from(move |_| callback.emit(()))
    };
    
    let cancel_callback = {
        let callback = props.on_cancel.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <div class="action-buttons">
            <button 
                class="load-button" 
                onclick={load_callback}
                disabled={props.creation_mode}
            >
                {"Load"}
            </button>
            {if !props.creation_mode {
                html! {
                    <button class="create-button" onclick={create_callback}>
                        {"Create"}
                    </button>
                }
            } else {
                html! {
                    <>
                        <button class="save-button" onclick={save_callback} disabled={props.saving}>
                            {if props.saving { "Saving..." } else { "Save" }}
                        </button>
                        <button class="cancel-button" onclick={cancel_callback}>
                            {"Cancel"}
                        </button>
                    </>
                }
            }}
        </div>
    }
} 