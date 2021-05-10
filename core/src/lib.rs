//use log::{info, trace, warn, error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// # Workspace definition file
///
/// Contains
pub struct Workspace {
    /// Company or CTF name
    #[serde(default = "default_name")]
    pub name: String,
    /// Company or CTF top url
    #[serde(default = "default_url")]
    pub url: String,

    #[serde(default)]
    pub settings: Settings,

}

fn default_name() -> String {
    "CorpName".to_string()
}

fn default_url() -> String {
    "https://www.example.com".to_string()
}

#[derive(Serialize, Deserialize)]
#[derive(PartialEq,Debug)]
pub enum FolderConvention {
    Name,
    IP,
    URL,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// This setting influences how folders are created
    pub preferred_folder_convention: FolderConvention,
    /// Time frequency to update the watcher process in ms
    pub update_time: u16,
}


impl Default for Settings {
    fn default() -> Settings {
        Settings {
            preferred_folder_convention: FolderConvention::IP,
            update_time: 100
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn deserialize_default_workspace() {
        let ws: Workspace = toml::from_str(r#"
            "#).unwrap();
            assert_eq!(ws.name, "CorpName");
            assert_eq!(ws.url, "https://www.example.com");
            assert_eq!(ws.settings.preferred_folder_convention, FolderConvention::IP);
            assert_eq!(ws.settings.update_time, 100);
    }

    #[test]
    fn deserialize_workspace() {
        let ws: Workspace = toml::from_str(r#"
            name = "Test"
            url = "https://www.example2.com"
            "#).unwrap();
        assert_eq!(ws.name, "Test");
        assert_eq!(ws.url, "https://www.example2.com");
        assert_eq!(ws.settings.update_time, 100);
    }

    #[test]
    fn deserialize_with_unknown_workspace() {
        let ws: Workspace = toml::from_str(r#"
            name = "Test"
            url = "https://www.example2.com"
            NotKnown = "Just a humble try"
            "#).unwrap();
        assert_eq!(ws.name, "Test");
        assert_eq!(ws.url, "https://www.example2.com");
        assert_eq!(ws.settings.update_time, 100);
    }

    #[test]
    fn serialize_workspace() {
        // let ws = Workspace {
        //     name: "Test".to_string(),
        //     url: "".to_string(),
        //     settings: Settings {}
        // };
        // let toml = toml::to_string(&ws).unwrap();
        // assert_eq!(toml, r#"name = "Test"
        //     "#);
    }
}
