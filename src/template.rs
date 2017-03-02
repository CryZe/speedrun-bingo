use serde_json::{self, Result};
use {generator, Mode, Bingo};

#[derive(Deserialize, Serialize)]
pub struct Template(pub Vec<Vec<Goal>>);

#[derive(Deserialize, Serialize)]
pub struct Goal {
    pub name: String,
    pub types: Vec<String>,
}

impl Template {
    pub fn generate(&self, seed: u32, mode: Mode) -> Bingo {
        generator::generate(seed, mode, self)
    }

    pub fn from_json_str(json: &str) -> Result<Self> {
        serde_json::from_str(json)
    }

    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self)
    }

    pub fn to_json_string_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json() {
        let template = Template(vec![vec![Goal {
                                              name: "Red Coin Star in WF".into(),
                                              types: vec!["WF".into()],
                                          }]]);

        assert_eq!(r#"[[{"name":"Red Coin Star in WF","types":["WF"]}]]"#,
                   template.to_json_string().unwrap());
    }

    #[test]
    fn from_json() {
        let sm64 = include_str!("templates/sm64.json");
        Template::from_json_str(sm64).unwrap();
    }
}
