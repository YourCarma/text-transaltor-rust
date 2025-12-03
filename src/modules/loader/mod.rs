pub mod errors;
pub mod models;

use std::env;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use serde_json::Value;

use crate::config::ServiceConfig;
use crate::modules::loader::models::units::{Language, TargetLanguage};
use crate::modules::loader::{errors::LoaderResult, models::units::ModelGarden};

pub async fn model_garden(server_config: &ServiceConfig) -> LoaderResult<ModelGarden> {
    const SOURCE_LANGUAGE_IDX: usize = 0;
    const TARGET_LANGUAGE_IDX: usize = 1;
    tracing::info!("Getting languages");
    // let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    // let file_path = Path::new(&manifest_dir).join("src/modules/loader/assets/iso639.json");
    let cwd = env::current_dir().unwrap();
    let file_path = Path::new(&cwd).join("assets/iso639.json");
    tracing::debug!("{:?}", file_path);
    let mut language_permutations: Vec<Vec<&String>> = server_config
        .server()
        .allowed_languages()
        .iter()
        .permutations(2)
        .collect();

    language_permutations.sort();
    let formalized_string = read_formalized_file(&file_path)?;
    let mut model_garden = ModelGarden::new();

    let deserialized_data: Value = serde_json::from_str(&formalized_string)?;

    drop(formalized_string);

    let deserialized_object = deserialized_data.as_object();
    if let Some(langs) = deserialized_object {
        for (index, allowed_language) in language_permutations.iter().enumerate() {
            let source_language = allowed_language[SOURCE_LANGUAGE_IDX];
            let target_language = allowed_language[TARGET_LANGUAGE_IDX];

            if index == 0 {
                let source_formalized_name =
                    langs[source_language]["name"].as_str().unwrap().to_string();
                let source_formalized_iso = langs[source_language]["639-1"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let mut language = Language::new(source_formalized_name, source_formalized_iso);

                let target_formalized_name =
                    langs[target_language]["name"].as_str().unwrap().to_string();
                let target_formalized_iso = langs[target_language]["639-1"]
                    .as_str()
                    .unwrap()
                    .to_string();

                let target_garden_language =
                    TargetLanguage::new(target_formalized_name, target_formalized_iso);

                language.add_target(target_garden_language);
                model_garden.add_language(language);
            }
            if index > 0 {
                let previous_language = language_permutations[index - 1][SOURCE_LANGUAGE_IDX];
                if previous_language != source_language {
                    let source_formalized_name =
                        langs[source_language]["name"].as_str().unwrap().to_string();
                    let source_formalized_iso = langs[source_language]["639-1"]
                        .as_str()
                        .unwrap()
                        .to_string();

                    let target_formalized_name =
                        langs[target_language]["name"].as_str().unwrap().to_string();
                    let target_formalized_iso = langs[target_language]["639-1"]
                        .as_str()
                        .unwrap()
                        .to_string();
                    let mut language = Language::new(source_formalized_name, source_formalized_iso);

                    let target_garden_language =
                        TargetLanguage::new(target_formalized_name, target_formalized_iso);
                    language.add_target(target_garden_language);
                    model_garden.add_language(language);
                } else {
                    let target_formalized_name =
                        langs[target_language]["name"].as_str().unwrap().to_string();
                    let target_formalized_iso = langs[target_language]["639-1"]
                        .as_str()
                        .unwrap()
                        .to_string();

                    let target_garden_language =
                        TargetLanguage::new(target_formalized_name, target_formalized_iso);
                    model_garden
                        .languages_mut()
                        .last_mut()
                        .unwrap()
                        .targets_mut()
                        .push(target_garden_language);
                }
            }
        }
    }
    Ok(model_garden)
}

fn read_formalized_file(formalized_file_path: &Path) -> LoaderResult<String> {
    let string_content = fs::read_to_string(formalized_file_path)?;
    Ok(string_content)
}

#[cfg(test)]
mod test_loader {
    use crate::{config::ServiceConfig, modules::loader::model_garden};

    #[tokio::test]
    async fn test_model_garden() -> Result<(), anyhow::Error> {
        let service_config = ServiceConfig::new()?;
        let _test_data = model_garden(&service_config).await?;

        Ok(())
    }
}
