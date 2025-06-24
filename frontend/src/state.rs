pub mod app_state {
    use crate::{ContentSource, ContentItem, AddContentType};
    use crate::services::api::{CoreGrammar, CommunityGrammar, UserExpression};

    #[derive(Debug, Clone)]
    pub enum AppMode {
        Browsing {
            source: ContentSource,
            selected_item: Option<ContentItem>,
            selected_system: i32,
        },
        Creating {
            source: ContentSource,
            add_type: AddContentType,
            name: String,
            selected_system: i32,
        },
        Loading {
            source: ContentSource,
        },
    }

    #[derive(Debug, Clone)]
    pub struct UIState {
        pub search_query: String,
        pub show_content_browser: bool,
        pub success_message: Option<String>,
        pub error: Option<String>,
    }

    impl Default for UIState {
        fn default() -> Self {
            Self {
                search_query: String::new(),
                show_content_browser: false,
                success_message: None,
                error: None,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct AppState {
        pub mode: AppMode,
        pub ui: UIState,
        pub core_grammars: Vec<CoreGrammar>,
        pub community_grammars: Vec<CommunityGrammar>,
        pub user_expressions: Vec<UserExpression>,
        pub filtered_content: Vec<ContentItem>,
    }

    impl Default for AppState {
        fn default() -> Self {
            Self {
                mode: AppMode::Browsing {
                    source: ContentSource::CoreGrammar,
                    selected_item: None,
                    selected_system: 1,
                },
                ui: UIState::default(),
                core_grammars: Vec::new(),
                community_grammars: Vec::new(),
                user_expressions: Vec::new(),
                filtered_content: Vec::new(),
            }
        }
    }

    impl AppState {
        // Mode transitions
        pub fn browse_content_source(&mut self, source: ContentSource) {
            self.mode = AppMode::Browsing {
                source,
                selected_item: None,
                selected_system: self.selected_system(),
            };
        }

        pub fn start_creation(&mut self, add_type: AddContentType) {
            if let AppMode::Browsing { source, selected_system, .. } = &self.mode {
                self.mode = AppMode::Creating {
                    source: *source,
                    add_type,
                    name: String::new(),
                    selected_system: *selected_system,
                };
            }
        }

        pub fn update_creation_name(&mut self, name: String) {
            if let AppMode::Creating { add_type, source, selected_system, .. } = &self.mode {
                self.mode = AppMode::Creating {
                    source: *source,
                    add_type: *add_type,
                    name,
                    selected_system: *selected_system,
                };
            }
        }

        pub fn complete_creation(&mut self) {
            if let AppMode::Creating { source, selected_system, .. } = &self.mode {
                self.mode = AppMode::Browsing {
                    source: *source,
                    selected_item: None,
                    selected_system: *selected_system,
                };
            }
        }

        pub fn cancel_creation(&mut self) {
            if let AppMode::Creating { source, selected_system, .. } = &self.mode {
                self.mode = AppMode::Browsing {
                    source: *source,
                    selected_item: None,
                    selected_system: *selected_system,
                };
            }
        }

        pub fn enter_loading(&mut self) {
            let source = self.current_content_source();
            self.mode = AppMode::Loading { source };
        }

        pub fn exit_loading(&mut self, source: ContentSource) {
            let selected_system = self.selected_system();
            self.mode = AppMode::Browsing {
                source,
                selected_item: None,
                selected_system,
            };
        }

        // Getters
        pub fn current_content_source(&self) -> ContentSource {
            match &self.mode {
                AppMode::Browsing { source, .. } => *source,
                AppMode::Creating { source, .. } => *source,
                AppMode::Loading { source } => *source,
            }
        }

        pub fn selected_item(&self) -> Option<&ContentItem> {
            match &self.mode {
                AppMode::Browsing { selected_item, .. } => selected_item.as_ref(),
                _ => None,
            }
        }

        pub fn selected_system(&self) -> i32 {
            match &self.mode {
                AppMode::Browsing { selected_system, .. } => *selected_system,
                AppMode::Creating { selected_system, .. } => *selected_system,
                AppMode::Loading { .. } => 1, // Default
            }
        }

        pub fn creation_details(&self) -> Option<(AddContentType, String)> {
            match &self.mode {
                AppMode::Creating { add_type, name, .. } => Some((*add_type, name.clone())),
                _ => None,
            }
        }

        pub fn is_loading(&self) -> bool {
            matches!(self.mode, AppMode::Loading { .. })
        }

        pub fn is_creating(&self) -> bool {
            matches!(self.mode, AppMode::Creating { .. })
        }

        // Setters
        pub fn select_item(&mut self, item: Option<ContentItem>) {
            if let AppMode::Browsing { source, selected_system, .. } = &self.mode {
                self.mode = AppMode::Browsing {
                    source: *source,
                    selected_item: item,
                    selected_system: *selected_system,
                };
            }
        }

        pub fn select_system(&mut self, system: i32) {
            match &self.mode {
                AppMode::Browsing { source, selected_item, .. } => {
                    self.mode = AppMode::Browsing {
                        source: *source,
                        selected_item: selected_item.clone(),
                        selected_system: system,
                    };
                }
                AppMode::Creating { source, add_type, name, .. } => {
                    self.mode = AppMode::Creating {
                        source: *source,
                        add_type: *add_type,
                        name: name.clone(),
                        selected_system: system,
                    };
                }
                _ => {}
            }
        }

        // UI state methods
        pub fn set_search_query(&mut self, query: String) {
            self.ui.search_query = query;
        }

        pub fn toggle_content_browser(&mut self) {
            self.ui.show_content_browser = !self.ui.show_content_browser;
        }

        pub fn set_success(&mut self, message: String) {
            self.ui.success_message = Some(message);
            self.ui.error = None;
        }

        pub fn set_error(&mut self, message: String) {
            self.ui.error = Some(message);
            self.ui.success_message = None;
        }

        pub fn clear_notifications(&mut self) {
            self.ui.success_message = None;
            self.ui.error = None;
        }
    }
} 