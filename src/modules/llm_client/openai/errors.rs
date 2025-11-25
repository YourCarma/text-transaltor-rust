use async_openai::error::OpenAIError;
use base64::DecodeError;
use std::io::Error as IOError;
use serde_json::Error as SerdeError;

use crate::modules::llm_client::errors::TranslatorErrors;
use crate::modules::llm_client::openai::models::errors::{OpenRouterError, OpenRouterErrorCodes};


impl From<OpenAIError> for TranslatorErrors {
    fn from(err: OpenAIError) -> Self {
        match err {
            OpenAIError::ApiError(_err) => TranslatorErrors::ServiceUnavailable("Open AI API".to_owned()),
            OpenAIError::InvalidArgument(err) => TranslatorErrors::AnotherError(err.to_string()),
            OpenAIError::FileReadError(err) | 
            OpenAIError::FileSaveError(err) => TranslatorErrors::IOError(err.to_string()),
            OpenAIError::Reqwest(err) => TranslatorErrors::RequestError(format!("Request Error: {}", err.to_string())),
            OpenAIError::JSONDeserialize(context, err) => {
                let error: Result<OpenRouterError, _> = serde_json::from_str(&err);
                match error{
                    Ok(err) => {
                        let error_code = err.error().code();
                        let error_msg = err.error().message();
                        match OpenRouterErrorCodes::from_status_code(error_code.to_owned()){
                            Some(OpenRouterErrorCodes::Unauthorized) => TranslatorErrors::Unauthorized(error_msg.to_owned()),
                            Some(OpenRouterErrorCodes::ModelModeration) => TranslatorErrors::ModelModerationError(error_msg.to_owned()),
                            Some(OpenRouterErrorCodes::RateLimit) => TranslatorErrors::RateLimited(error_msg.to_owned()),
                            Some(OpenRouterErrorCodes::ServiceUnavailable) => TranslatorErrors::ServiceUnavailable(error_msg.to_owned()),
                            Some(OpenRouterErrorCodes::NoCredits) => TranslatorErrors::NoCredits(error_msg.to_owned()),
                            Some(OpenRouterErrorCodes::BadRequest) => TranslatorErrors::BadRequest(error_msg.to_owned()),
                            Some(OpenRouterErrorCodes::InvalidResponse) => TranslatorErrors::InvalidResponse(error_msg.to_owned()),
                            Some(OpenRouterErrorCodes::Timeout) => TranslatorErrors::Timeout(error_msg.to_owned()),

                            None => TranslatorErrors::AnotherError(format!("Unknown error code: {}", error_msg))
                        }
                    },
                    Err(err) => TranslatorErrors::AnotherError(format!("Error on Deserialize request: context: {context},\n error: {err}",
                                                                                context=context.to_string(), err=err.to_string()))

                }
                
            },
            _ => TranslatorErrors::AnotherError(format!("OpenAI Generation Error: {}", err.to_string())),
        }
    }
}

impl From<DecodeError> for TranslatorErrors {
    fn from(err: DecodeError) -> Self {
        match err {
            _ => TranslatorErrors::IOError(format!("Error of decoding image: {}", err.to_string()))
        }
    }
}

impl  From<IOError> for TranslatorErrors{
    fn from(err: IOError) -> Self {
        match err {
            _ => TranslatorErrors::IOError(format!("Error of saving image: {}", err.to_string()))
        }
    }
    
}

impl From<SerdeError> for TranslatorErrors {
    fn from(err: SerdeError) -> Self {
        match err {
        _ => TranslatorErrors::DeserializeError(format!("Deserialize Error: {}", err.to_string()))
        }
    }
    
}