use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;

const OPENAI_COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";
const DEFAULT_TEMPERATURE: f64 = 0.7;
const DEFAULT_MODEL: Model = Model::Gpt3_5Turbo;

#[derive(Serialize)]
enum Model {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
}

#[derive(Serialize, Deserialize)]
enum Role {
    #[serde(rename = "user")]
    User,

    #[serde(rename = "system")]
    System,

    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: Role,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    index: u64,
    message: Message,
    finish_reason: String, // could be more strongly typed
}

#[derive(Serialize, Deserialize)]
struct UsageInfo {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Serialize, Deserialize)]
struct SuccessfullConversationResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: UsageInfo,
}

#[derive(Serialize)]
struct OpenAiConversationRequest {
    model: Model,
    messages: Vec<Message>,
    temperature: f64,
}

const API_KEY_ENV_NAME: &str = "OPENAI_API_KEY";
enum AskError {
    RequestError(reqwest::Error),
    WrongApiKey, // TODO: catch that case after getting result & error on successfull parsing
    NotEnoughtCredit, // TODO: catch that case after getting result & error on successfull parsing
    ParsingError(serde_json::Error),
}

fn ask_one(
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

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Question to ask GTP
    #[arg(short, long)]
    question: String,

    /// Temperature (how "imaginative"/"out of touch" the NN is) of the GTP
    #[arg(short, long, default_value_t = 0.7)]
    temperature: f64,
}

fn main() -> serde_json::Result<()> {
    let args = Args::parse();
    let (question, temperature) = (args.question, args.temperature);

    let response = ask_one(Some(Model::Gpt3_5Turbo), question, Some(temperature));

    let successfull = match response {
        Ok(res) => res,
        Err(e) => match e {
            AskError::ParsingError(err) => {
                println!("Parsing error: {}", err);
                panic!("<Parsing error>");
            }
            _ => {
                panic!("<Default error>")
            } // TODO: more error handling
        },
    };

    let text_response = successfull
        .choices
        .get(0)
        .expect("[There were no response]");

    println!("{}", text_response.message.content);

    return Ok(());
}
