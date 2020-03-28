use coi::{Container, Inject};
use std::sync::Arc;

pub struct Context {
    container: Arc<Container>,
}

impl Context {
    pub fn new(container: Arc<Container>) -> Self {
        Context {
            container: container,
        }
    }

    pub fn resolve<T>(&self, name: &'static str) -> Arc<T>
    where
        T: Inject + ?Sized,
    {
        self.container.resolve::<T>(name).expect("Should exist")
    }
}

impl juniper::Context for Context {}
