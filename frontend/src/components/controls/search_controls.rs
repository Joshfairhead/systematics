use yew::prelude::*;
use crate::ContentSource;
use super::{ContentSourceTabs, ActionButtons};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content_source: ContentSource,
    pub creation_mode: bool,
    pub saving: bool,
    pub on_core_selected: Callback<()>,
    pub on_community_selected: Callback<()>,
    pub on_user_selected: Callback<()>,
    pub on_load: Callback<()>,
    pub on_create: Callback<()>,
    pub on_save: Callback<()>,
    pub on_cancel: Callback<()>,
}

#[function_component(SearchControls)]
pub fn search_controls(props: &Props) -> Html {
    html! {
        <div class="search-controls">
            <div class="search-bar">
                <ContentSourceTabs
                    content_source={props.content_source}
                    creation_mode={props.creation_mode}
                    on_core_selected={props.on_core_selected.clone()}
                    on_community_selected={props.on_community_selected.clone()}
                    on_user_selected={props.on_user_selected.clone()}
                />
                
                <ActionButtons
                    creation_mode={props.creation_mode}
                    saving={props.saving}
                    on_load={props.on_load.clone()}
                    on_create={props.on_create.clone()}
                    on_save={props.on_save.clone()}
                    on_cancel={props.on_cancel.clone()}
                />
            </div>
        </div>
    }
} 