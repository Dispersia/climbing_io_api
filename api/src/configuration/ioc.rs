use coi::{container, Container};
use services::JournalServiceProvider;
use shared::ConfigProvider;

pub fn create_container() -> Container {
    container! {
        config => ConfigProvider; singleton,
        journal_service => JournalServiceProvider; singleton
    }
}
