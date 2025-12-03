use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, Getters, ToSchema, Debug)]
#[getset(get = "pub")]
pub struct TargetLanguage {
    name: String,
    iso: String
}


#[derive(Serialize, Deserialize, Getters, ToSchema, Debug, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Language {
    name: String,
    iso: String,
    #[serde(default)]
    targets: Vec<TargetLanguage>
}

#[derive(Serialize, Deserialize, Getters, ToSchema, Debug, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct ModelGarden {
    languages: Vec<Language>,
}


impl Language {
    pub fn new(name: String, iso: String) -> Self{
        Self { name, iso, targets: Vec::new() }
    }

    pub fn add_target(&mut self, target: TargetLanguage) {
        self.targets.push(target);
    }
}

impl TargetLanguage {
    pub fn new(name: String, iso: String) -> Self{
        Self { name, iso }
    }
}

impl ModelGarden {
    pub fn new() -> Self{
        Self { languages: Vec::new() }
    }

     pub fn add_language(&mut self, language: Language) {
        self.languages.push(language);
    }
}