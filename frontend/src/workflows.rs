pub mod creation_workflow {
    use crate::AddContentType;

    #[derive(Debug, Clone)]
    pub struct CreationWorkflow {
        pub content_type: AddContentType,
        pub name: String,
        pub user_inputs: Vec<String>,
    }

    impl CreationWorkflow {
        pub fn new(content_type: AddContentType, name: String, term_count: usize) -> Self {
            Self {
                content_type,
                name,
                user_inputs: vec![String::new(); term_count],
            }
        }

        pub fn update_input(&mut self, index: usize, value: String) {
            if index < self.user_inputs.len() {
                self.user_inputs[index] = value;
            }
        }

        pub fn is_complete(&self) -> bool {
            self.user_inputs.iter().all(|input| !input.trim().is_empty())
        }

        pub fn can_create_in_source(source: crate::ContentSource) -> bool {
            use crate::ContentSource;
            match source {
                ContentSource::CoreGrammar => false, // Read-only
                ContentSource::CommunityGrammar | ContentSource::UserExpressions => true,
            }
        }

        pub fn validate_creation_readiness(
            _source: crate::ContentSource,
            add_type: Option<AddContentType>,
            name: Option<&String>,
        ) -> Result<(), String> {
            if let Some(name) = name {
                if name.trim().is_empty() {
                    return Err("Name cannot be empty".to_string());
                }
            } else {
                return Err("Name is required".to_string());
            }

            if add_type.is_none() {
                return Err("Content type is required".to_string());
            }

            Ok(())
        }
    }
} 