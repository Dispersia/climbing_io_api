use super::Tag;
use crate::Context;
use services::TagServiceTrait;

/// Annotates basic properties of an exercise
pub struct Exercise {
    id: juniper::ID,
    name: String,
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

    async fn tags(&self, context: &Context) -> Vec<Tag> {
        let tag_service = context.resolve::<dyn TagServiceTrait>("tag_service");

        match self.id.parse::<i32>() {
            Ok(id) => tag_service
                .get_tags_by_exercise(id)
                .await
                .into_iter()
                .map(|x| x.into())
                .collect(),
            Err(_) => vec![],
        }
    }
}

impl From<services::Exercise> for Exercise {
    fn from(exercise: services::Exercise) -> Exercise {
        Exercise {
            id: juniper::ID::new(exercise.id.to_string()),
            name: exercise.name,
        }
    }
}
