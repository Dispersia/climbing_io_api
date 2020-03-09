use coi::{container, Container};
use shared::ConfigProvider;

pub fn create_container() -> Container {
    container! {
        config => ConfigProvider; singleton
    }
}
