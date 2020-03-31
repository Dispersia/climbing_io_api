use super::Tag;
use crate::{error::GraphQLError, Context};
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

    async fn tags(&self, context: &Context) -> Result<Vec<Tag>, GraphQLError> {
        let tag_service = context.resolve::<dyn TagServiceTrait>("tag_service");

        match self.id.parse::<i32>() {
            Ok(_id) => tag_service
                .get_tags()
                .await
                .map(|x| x.into_iter().map(Into::into).collect())
                .map_err(|e| {
                    log::error!("{}", e);
                    GraphQLError::GenericFailure
                }),
            Err(_) => Err(GraphQLError::GenericFailure),
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
