use coi::Inject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn load_config() -> Config {
    let files = get_files();
    let mut config: Value =
        serde_json::from_str(files[0]).expect("Could not read appSettings.json");

    for file in files.iter().skip(1) {
        let content = serde_json::from_str(file).expect("Could not read settings file.");
        merge_json(&mut config, content);
    }

    serde_json::from_value(config).expect("Unable to read configuration files.")
}

#[derive(Inject, Serialize, Deserialize, Debug)]
#[coi(provides pub Config with load_config())]
pub struct Config {
    pub id: i32,
    pub name: String,
}

fn get_files<'a>() -> Vec<&'a str> {
    let content = include_str!("../../appSettings.json");
    let mut list = vec![content];

    #[cfg(feature = "dev")]
    {
        let content = include_str!("../../appSettings.Dev.json");
        list.push(content);
    }

    #[cfg(feature = "prod")]
    {
        let content = include_str!("../../appSettings.Prod.json");
        list.push(content);
    }

    list
}

fn merge_json(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge_json(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}
