use core::fmt;
use serde::{Deserialize, Serialize};
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
            _ => write!(f, "{}", "<default error>"),
        }
    }
}

pub fn ask_one(
    model: Option<Model>,
    message: String,
    temperature: Option<f64>,
) -> Result<SuccessfullConversationResponse, AskError> {
    let messages = vec![Message {
        content: message,
        role: Role::User,
    }];

    return send_to_open_ai(None, model, temperature, messages);
}

fn send_to_open_ai(
    api_key: Option<String>,
    model: Option<Model>,
    temperature: Option<f64>,
    messages: Vec<Message>,
) -> Result<SuccessfullConversationResponse, AskError> {
    let api_key = api_key.unwrap_or_else(|| env::var(API_KEY_ENV_NAME)
                                 // If API key not provided, try to read it from the env variable
                                 .expect(&format!(
                                         "The enviroment variable {} was not set. Please, set the {} enviroment variable: (e.g. \n\texport {}=\"<KEY>\"",
                                         API_KEY_ENV_NAME, API_KEY_ENV_NAME, API_KEY_ENV_NAME))
                                 );

    let model = match model {
        Some(m) => m,
        None => DEFAULT_MODEL,
    };

    let temperature = match temperature {
        Some(t) => t,
        None => DEFAULT_TEMPERATURE,
    };

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

const CLI_ASKING_PREMESSAGE: &str = "You are the console application that generates propositions for calling command line applications based on user request. You should answer the user just with one line of script that, as you think, the most fittingly does what user requested. DO NOT add some sort of comments, explanations etc. Just the described script.";

pub fn ask_cli(
    model: Option<Model>,
    description: String,
    temperature: Option<f64>,
) -> Result<SuccessfullConversationResponse, AskError> {
    let system_message: Message = Message {
        content: CLI_ASKING_PREMESSAGE.to_string(),
        role: Role::System,
    };

    let messages = vec![
        system_message,
        Message {
            content: description,
            role: Role::User,
        },
    ];

    return send_to_open_ai(None, model, temperature, messages);
}
