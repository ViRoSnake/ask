pub mod openai;
use crate::openai::{ask_sync, AskError, Message, Model, Role, SuccessfullConversationResponse};

const DEFAULT_TEMPERATURE: f64 = 0.7;
const DEFAULT_MODEL: Model = Model::Gpt3_5Turbo;
fn default_ask(
    api_key: String,
    model: Option<Model>,
    messages: Vec<Message>,
    temperature: Option<f64>,
) -> Result<SuccessfullConversationResponse, AskError> {
    let model = match model {
        Some(m) => m,
        None => DEFAULT_MODEL,
    };

    let temperature = match temperature {
        Some(t) => t,
        None => DEFAULT_TEMPERATURE,
    };
    return ask_sync(api_key, model, temperature, messages);
}
pub fn ask_question_sync(
    api_key: String,
    model: Option<Model>,
    message: String,
    temperature: Option<f64>,
) -> Result<SuccessfullConversationResponse, AskError> {
    let messages = vec![Message {
        content: message,
        role: Role::User,
    }];

    return default_ask(api_key, model, messages, temperature);
}

const CLI_ASKING_PREMESSAGE: &str = "You are the console application that generates propositions for calling command line applications based on user request. You should answer the user just with one line of script that, as you think, the most fittingly does what user requested. DO NOT add some sort of comments, explanations etc. Just the described script.";
pub fn ask_cli(
    api_key: String,
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

    return default_ask(api_key, model, messages, temperature);
}
