use coi::Inject;

pub trait Trait1: Inject {
    fn describe(&self) -> &'static str;
}

#[derive(Inject)]
#[coi(provides pub dyn Trait1 with Impl1)]
struct Impl1;

impl Trait1 for Impl1 {
    fn describe(&self) -> &'static str {
        "I'm Still an impl!"
    }
}
