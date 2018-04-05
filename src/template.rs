#[cfg(not(feature = "std"))]
use arrayvec::{ArrayString, ArrayVec};
#[cfg(feature = "std")]
use serde_json::{de, ser, Result as DeResult, Result as SerResult};
#[cfg(feature = "json_core")]
use serde_json::de::{self, Result as DeResult};
use {generator, Bingo, Mode};

#[cfg(feature = "std")]
#[derive(Deserialize, Serialize)]
pub struct Template(pub Vec<Vec<Goal>>);

#[cfg(feature = "std")]
#[derive(Deserialize, Serialize)]
pub struct Goal {
    pub name: String,
    pub types: Vec<String>,
}

#[cfg(not(feature = "std"))]
#[derive(Deserialize, Serialize)]
pub struct Template(pub ArrayVec<[ArrayVec<[Goal; 10]>; 32]>);

#[cfg(not(feature = "std"))]
#[derive(Deserialize, Serialize)]
pub struct Goal {
    pub name: ArrayString<[u8; 256]>,
    pub types: ArrayVec<[ArrayString<[u8; 32]>; 8]>,
}

impl Template {
    pub fn generate(&self, seed: u32, mode: Mode) -> Bingo {
        generator::generate(seed, mode, self)
    }

    #[cfg(any(feature = "std", feature = "json_core"))]
    pub fn from_json_str(json: &str) -> DeResult<Self> {
        de::from_str(json)
    }

    #[cfg(feature = "std")]
    pub fn to_json_string(&self) -> SerResult<String> {
        ser::to_string(self)
    }

    #[cfg(feature = "std")]
    pub fn to_json_string_pretty(&self) -> SerResult<String> {
        ser::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json() {
        let template = Template(vec![
            vec![
                Goal {
                    name: "Red Coin Star in WF".into(),
                    types: vec!["WF".into()],
                },
            ],
        ]);

        assert_eq!(
            r#"[[{"name":"Red Coin Star in WF","types":["WF"]}]]"#,
            template.to_json_string().unwrap()
        );
    }

    #[test]
    fn from_json() {
        let sm64 = include_str!("templates/sm64.json");
        Template::from_json_str(sm64).unwrap();
    }
}
