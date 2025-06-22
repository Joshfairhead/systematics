use yew::prelude::*;
use crate::ContentSource;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content_source: ContentSource,
    pub creation_mode: bool,
    pub on_core_selected: Callback<()>,
    pub on_community_selected: Callback<()>,
    pub on_user_selected: Callback<()>,
}

#[function_component(ContentSourceTabs)]
pub fn content_source_tabs(props: &Props) -> Html {
    let core_callback = {
        let callback = props.on_core_selected.clone();
        Callback::from(move |_| callback.emit(()))
    };
    
    let community_callback = {
        let callback = props.on_community_selected.clone();
        Callback::from(move |_| callback.emit(()))
    };
    
    let user_callback = {
        let callback = props.on_user_selected.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <div class="content-source-tabs">
            <button 
                class={classes!("tab-button", if props.content_source == ContentSource::CoreGrammar { "active" } else { "" })}
                onclick={core_callback}
                disabled={props.creation_mode}
            >
                {"Core"}
            </button>
            <button 
                class={classes!("tab-button", if props.content_source == ContentSource::CommunityGrammar { "active" } else { "" })}
                onclick={community_callback}
                disabled={props.creation_mode}
            >
                {"Community"}
            </button>
            <button 
                class={classes!("tab-button", if props.content_source == ContentSource::UserExpressions { "active" } else { "" })}
                onclick={user_callback}
                disabled={props.creation_mode}
            >
                {"User"}
            </button>
        </div>
    }
} 