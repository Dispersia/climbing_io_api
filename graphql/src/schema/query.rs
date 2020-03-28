use coi::{Container, Inject};
use juniper::FieldResult;
use services::JournalServiceTrait;
use std::sync::Arc;

#[derive(juniper::GraphQLEnum)]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct Context {
    container: Arc<Container>,
}

impl Context {
    pub fn new(container: Arc<Container>) -> Self {
        Context {
            container: container,
        }
    }

    fn resolve<T>(&self, name: &'static str) -> Arc<T>
    where
        T: Inject + ?Sized,
    {
        self.container.resolve::<T>(name).expect("Should exist")
    }
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(
    Context = Context,
)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    async fn human(context: &Context, id: String) -> FieldResult<Human> {
        let service = context.resolve::<dyn JournalServiceTrait>("journal_service");
        let name = service.test_get().await;

        Ok(Human {
            id: id.to_string(),
            name: name,
            appears_in: vec![Episode::Jedi],
            home_planet: "Earth".to_string(),
        })
    }
}

pub struct Mutation;

#[juniper::graphql_object(
    Context = Context,
)]
impl Mutation {
    fn createHuman(_context: &Context, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: 1.to_string(),
            name: new_human.name.to_string(),
            appears_in: vec![Episode::Jedi],
            home_planet: new_human.home_planet.to_string(),
        })
    }
}
