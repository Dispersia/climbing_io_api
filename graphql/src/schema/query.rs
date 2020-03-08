use juniper::FieldResult;

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

pub struct Context {}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn human(_context: &Context, id: String) -> FieldResult<Human> {
        Ok(Human {
            id: id.to_string(),
            name: "Aaron Housh".to_string(),
            appears_in: vec![Episode::Jedi],
            home_planet: "Earth".to_string(),
        })
    }
}

pub struct Mutation;

#[juniper::object(
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
