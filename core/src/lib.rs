use log::{info, trace, warn, error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Project {
    name: String
}


#[cfg(test)]
mod tests {
    use crate::Project;

    #[test]
    fn deserialize_project() {
        let project: Project = toml::from_str(r#"
            name = "Test"
            "#).unwrap();
            assert_eq!(project.name, "Test");
    }

    #[test]
    fn serialize_project() {
        let project = Project { name: "Test".to_string() };
        let toml = toml::to_string(&project).unwrap();
        assert_eq!(toml, r#"name = "Test"
"#);
    }
}
