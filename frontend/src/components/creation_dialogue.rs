use yew::{html, Component, Context, Html, Properties, Callback, TargetCast};
use web_sys::{HtmlInputElement, InputEvent};
use crate::{AddContentType, ContentSource};

#[derive(Properties, PartialEq)]
pub struct CreationDialogueProps {
    pub show: bool,
    pub content_source: ContentSource,
    pub on_create: Callback<(AddContentType, String)>,
    pub on_cancel: Callback<()>,
}

pub enum CreationDialogueMsg {
    SelectContentType(AddContentType),
    UpdateName(String),
    Create,
    Cancel,
}

pub struct CreationDialogue {
    selected_type: Option<AddContentType>,
    name: String,
}

impl Component for CreationDialogue {
    type Message = CreationDialogueMsg;
    type Properties = CreationDialogueProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_type: None,
            name: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreationDialogueMsg::SelectContentType(content_type) => {
                self.selected_type = Some(content_type);
                true
            }
            CreationDialogueMsg::UpdateName(name) => {
                self.name = name;
                true
            }
            CreationDialogueMsg::Create => {
                if let Some(content_type) = self.selected_type {
                    if !self.name.trim().is_empty() {
                        ctx.props().on_create.emit((content_type, self.name.clone()));
                        // Reset state
                        self.selected_type = None;
                        self.name.clear();
                    }
                }
                true
            }
            CreationDialogueMsg::Cancel => {
                // Reset state
                self.selected_type = None;
                self.name.clear();
                ctx.props().on_cancel.emit(());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !ctx.props().show {
            return html! {};
        }

        let content_source_name = match ctx.props().content_source {
            ContentSource::CommunityGrammar => "Community Grammar",
            ContentSource::UserExpressions => "User Expressions",
            ContentSource::CoreGrammar => "Core Grammar", // Won't show this
        };

        html! {
            <div class="creation-dialogue-overlay">
                <div class="creation-dialogue">
                    <div class="dialogue-header">
                        <h3>{"Create New Content"}</h3>
                        <p>{format!("in {}", content_source_name)}</p>
                    </div>
                    
                    <div class="content-type-selection">
                        <h4>{"What would you like to create?"}</h4>
                        <div class="content-type-grid">
                            {self.render_content_type_card(ctx, AddContentType::System, "System", "📐", "Create a systematic structure with your own terms")}
                            {self.render_content_type_card(ctx, AddContentType::Collection, "Collection", "📚", "Organize systematic content into collections")}
                            {self.render_content_type_card(ctx, AddContentType::Paper, "Paper", "📄", "Research paper exploring systematic relationships")}
                            {self.render_content_type_card(ctx, AddContentType::SystemCollection, "System Collection", "🔄", "Walk through systematic structures 1-8 in sequence")}
                            {self.render_content_type_card(ctx, AddContentType::Module, "Module", "🎓", "Educational module with systematic progression")}
                            {self.render_content_type_card(ctx, AddContentType::Book, "Book", "📖", "Comprehensive exploration of systematic principles")}
                        </div>
                    </div>
                    
                    {if self.selected_type.is_some() {
                        self.render_name_input(ctx)
                    } else {
                        html! {}
                    }}
                    
                    <div class="dialogue-actions">
                        <button 
                            class="cancel-button"
                            onclick={ctx.link().callback(|_| CreationDialogueMsg::Cancel)}
                        >
                            {"Cancel"}
                        </button>
                        <button 
                            class="create-button"
                            disabled={self.selected_type.is_none() || self.name.trim().is_empty()}
                            onclick={ctx.link().callback(|_| CreationDialogueMsg::Create)}
                        >
                            {"Create"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}

impl CreationDialogue {
    fn render_content_type_card(&self, ctx: &Context<Self>, content_type: AddContentType, title: &str, icon: &str, description: &str) -> Html {
        let is_selected = self.selected_type == Some(content_type);
        let class = if is_selected { "content-type-card selected" } else { "content-type-card" };
        
        html! {
            <div 
                class={class}
                onclick={ctx.link().callback(move |_| CreationDialogueMsg::SelectContentType(content_type))}
            >
                <div class="card-icon">{icon}</div>
                <div class="card-content">
                    <h5>{title}</h5>
                    <p>{description}</p>
                </div>
                {if is_selected {
                    html! { <div class="selection-indicator">{"✓"}</div> }
                } else {
                    html! {}
                }}
            </div>
        }
    }
    
    fn render_name_input(&self, ctx: &Context<Self>) -> Html {
        let content_type_name = match self.selected_type {
            Some(AddContentType::System) => "System",
            Some(AddContentType::Collection) => "Collection",
            Some(AddContentType::Paper) => "Paper",
            Some(AddContentType::SystemCollection) => "System Collection",
            Some(AddContentType::Module) => "Module",
            Some(AddContentType::Book) => "Book",
            Some(AddContentType::Definition) => "Definition",
            None => "Content",
        };
        
        html! {
            <div class="name-input-section">
                <h4>{format!("Name your {}", content_type_name)}</h4>
                <input 
                    type="text"
                    class="name-input"
                    placeholder={format!("Enter {} name...", content_type_name)}
                    value={self.name.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input = e.target_unchecked_into::<HtmlInputElement>();
                        CreationDialogueMsg::UpdateName(input.value())
                    })}
                    autofocus={true}
                />
            </div>
        }
    }
} 