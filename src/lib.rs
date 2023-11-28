
use serde::{Deserialize, Serialize};
use core::fmt;
use std::env;

const OPENAI_COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";
const DEFAULT_TEMPERATURE: f64 = 0.7;
const DEFAULT_MODEL: Model = Model::Gpt3_5Turbo;

#[derive(Serialize)]
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
}

#[derive(Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "user")]
    User,

    #[serde(rename = "system")]
    System,

    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    index: u64,
    pub message: Message,
    finish_reason: String, // could be more strongly typed
}

#[derive(Serialize, Deserialize)]
pub struct UsageInfo {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Serialize, Deserialize)]
pub struct SuccessfullConversationResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    pub choices: Vec<Choice>,
    usage: UsageInfo,
}

#[derive(Serialize)]
struct OpenAiConversationRequest {
    model: Model,
    messages: Vec<Message>,
    temperature: f64,
}

const API_KEY_ENV_NAME: &str = "OPENAI_API_KEY";
pub enum AskError {
    RequestError(reqwest::Error),
    WrongApiKey, // TODO: catch that case after getting result & error on successfull parsing
    NotEnoughtCredit, // TODO: catch that case after getting result & error on successfull parsing
    ParsingError(serde_json::Error),
}
impl fmt::Debug for AskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AskError::RequestError(re) => write!(f, "{}", re),
            AskError::ParsingError(pe) => write!(f, "{}", pe),
            _ => write!(f, "{}", "<default error>") 
        }
    }
}

pub fn ask_one(
    model: Option<Model>,
    message: String,
    temperature: Option<f64>,
) -> Result<SuccessfullConversationResponse, AskError> {
    let err_str = format!("The enviroment variable {} was not set. Please, set the {} enviroment variable: (e.g. \n\texport {}=\"<KEY>\"", API_KEY_ENV_NAME, API_KEY_ENV_NAME, API_KEY_ENV_NAME);
    let api_key = env::var(API_KEY_ENV_NAME).expect(&err_str);

    let model = match model {
        Some(m) => m,
        None => DEFAULT_MODEL,
    };

    let temperature = match temperature {
        Some(t) => t,
        None => DEFAULT_TEMPERATURE,
    };

    let client = reqwest::blocking::Client::new();

    let openai_request = OpenAiConversationRequest {
        model,
        temperature,
        messages: vec![Message {
            content: message,
            role: Role::User,
        }],
    };

    let body = client
        .post(OPENAI_COMPLETIONS_URL)
        .bearer_auth(api_key)
        .json(&openai_request)
        .send();

    match body {
        Ok(res) => {
            let text = match res.text() {
                Ok(str) => str,
                Err(e) => return Err(AskError::RequestError(e)),
            };

            match serde_json::from_str::<SuccessfullConversationResponse>(&text) {
                Ok(response) => return Ok(response),
                Err(parsing_error) => return Err(AskError::ParsingError(parsing_error)),
            }
        }
        Err(e) => return Err(AskError::RequestError(e)),
    };
}
