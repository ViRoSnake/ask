use core::fmt;

use serde::{Deserialize, Serialize};

const OPENAI_COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";
pub fn ask_sync(
    api_key: String,
    model: Model,
    temperature: f64,
    messages: Vec<Message>,
) -> Result<SuccessfullConversationResponse, AskError> {
    let openai_request = OpenAiConversationRequest {
        model,
        temperature,
        messages,
    };

    let client = reqwest::blocking::Client::new();
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

#[derive(Serialize, Clone)]
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
}

impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Model::Gpt3_5Turbo => write!(f, "{}", "3.5"),
        }
    }
}
impl std::str::FromStr for Model {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // TODO: rewrite to just something like "string X contains "3", "5",
            // "turbo"
            "3.5" => Ok(Model::Gpt3_5Turbo),
            "3.5-turbo" => Ok(Model::Gpt3_5Turbo),
            "3_5" => Ok(Model::Gpt3_5Turbo),
            "3_5_turbo" => Ok(Model::Gpt3_5Turbo),
            _ => Err("No such model yet supported"),
        }
    }
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
            _ => write!(f, "{}", "<default error>"),
        }
    }
}
