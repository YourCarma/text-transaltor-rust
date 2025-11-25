use async_openai::types::Role;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, Default, Debug)]
#[getset(get="pub", set="pub")]
pub struct ImageGenerationPayload{
    model: String,
    messages: Vec<Message>,
    modalities: Vec<Modality>
}

#[derive(Serialize, Deserialize, Getters, Default, Debug)]
pub struct Message{
    role: Role,
    content: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Modality{
    Image,
    Text
}



impl ImageGenerationPayload  {
    pub fn new(model: String, messages: Vec<Message>, modalities: Vec<Modality>) -> Self {
        Self {  
            model: model,
            messages: messages,
            modalities: modalities
        }
    }

    pub fn default_modalities() -> Vec<Modality>{
        vec![Modality::Text, Modality::Image]
    }
}

impl Message {
    pub fn new(role: Role, prompt: String) -> Self{
    Self{
        role: role,
        content: prompt
    }
    }
}
