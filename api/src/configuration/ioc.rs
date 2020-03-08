use coi::{container, Container};
use services::Impl1Provider;

pub fn create_container() -> Container {
    container! {
        trait1 => Impl1Provider
    }
}
