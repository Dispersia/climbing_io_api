use coi::{container, Container};
use services::Impl1Provider;
use shared::ConfigProvider;

pub fn create_container() -> Container {
    container! {
        config => ConfigProvider; singleton,
        trait1 => Impl1Provider
    }
}
