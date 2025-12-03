use std::fmt;

use getset::{Getters, Setters};
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Getters, Setters, PartialEq, Debug, Clone, ToSchema)]
#[getset(get = "pub", set = "pub")]
pub struct TranslateTask {
    #[serde(deserialize_with = "trim_string")]
    #[schema(default = "en")]
    source_language: String,
    #[serde(deserialize_with = "trim_string")]
    #[schema(default = "ru")]
    target_language: String,
    #[serde(deserialize_with = "trim_string")]
    #[schema(
        default = r"This is my rifle. There are many like it, but this one is mine.
My rifle is my best friend. It is my life. I must master it as I must master my life.
My rifle, without me, is useless. Without my rifle, I am useless. I must fire my rifle true.
I must shoot straighter than my enemy who is trying to kill me. I must shoot him before he shoots me. I will...
My rifle and I know that what counts in war is not the rounds we fire, the noise of our burst, nor the smoke we make.
We know that it is the hits that count. We will hit.."
    )]
    text: String,
}

impl Default for TranslateTask {
    fn default() -> Self {
        Self {
        source_language:  "en".to_owned(),
        target_language: "ru".to_owned(),
        text: r"This is my rifle. There are many like it, but this one is mine.
My rifle is my best friend. It is my life. I must master it as I must master my life.
My rifle, without me, is useless. Without my rifle, I am useless. I must fire my rifle true.
I must shoot straighter than my enemy who is trying to kill me. I must shoot him before he shoots me. I will...
My rifle and I know that what counts in war is not the rounds we fire, the noise of our burst, nor the smoke we make.
We know that it is the hits that count. We will hit...".to_owned(),
        }
    }
}

impl fmt::Display for TranslateTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TranslateTask:\n
            Source Language: \"{}\"\n
            Target Language: \"{}\"\n
            Text: \"{}\"",
            self.source_language, self.target_language, self.text
        )
    }
}

fn trim_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_string())
}
