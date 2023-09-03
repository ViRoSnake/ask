use serde::{Serialize, Deserialize};
use clap::Parser;

const OPENAI_API_KEY: &str = "<API_KEY>";
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
    finish_reason: String // could be more strongly typed
}

#[derive(Serialize, Deserialize)]
struct UsageInfo {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64
}

#[derive(Serialize, Deserialize)]
struct SuccessfullConversationResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: UsageInfo 
}

#[derive(Serialize)]
struct OpenAiConversationRequest {
    model: Model,
    messages: Vec<Message>,
    temperature: f64,
}

fn ask_one(
    model: Option<Model>,
    message: String,
    temperature: Option<f64>,
) -> Result<String, reqwest::Error> {
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
        .bearer_auth(OPENAI_API_KEY)
        .json(&openai_request)
        .send()?
        .text();

    match body {
        Ok(s) => return Ok(s),
        Err(e) => {
            println!("{}", e);
            return Err(e);
        },
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
    let (question, temperature ) = (args.question, args.temperature);

    let response = ask_one(
        Some(Model::Gpt3_5Turbo),
        question,
        Some(temperature),
    ).unwrap();

    let successfull_response: SuccessfullConversationResponse = 
    serde_json::from_str(&response).unwrap();

    let text_response = successfull_response.choices
    .get(0)
    .expect("[There were no response]");

    let m = &text_response.message.content;
    let n = m.clone();

    println!("{}", n);
            
    return Ok(());
}
