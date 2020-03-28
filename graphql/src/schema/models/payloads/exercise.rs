use super::Tag;
use crate::Context;

/// Annotates basic properties of an exercise
pub struct Exercise {
    id: juniper::ID,
    name: String,
}

impl Exercise {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: juniper::ID::new(id),
            name: name.to_string(),
        }
    }
}

#[juniper::graphql_object(
    Context = Context
)]
impl Exercise {
    fn id(&self) -> &juniper::ID {
        &self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn tags(_context: &Context) -> Vec<Tag> {
        vec![
            Tag {
                id: juniper::ID::new("1"),
                name: "Workout".to_string(),
            },
            Tag {
                id: juniper::ID::new("2"),
                name: "Workout2".to_string(),
            },
        ]
    }
}
