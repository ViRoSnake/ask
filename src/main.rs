use std::env;

use ask::openai::{AskError, Model};
use ask::{ask_cli, ask_question_sync};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// OpenAI API key. If not included, the program will try to read OPENAI_API_KEY enviroment variable
    #[arg(short, long)]
    key: Option<String>,

    /// Question to ask GTP
    #[arg(short, long)]
    question: Option<String>,

    /// Temperature (how "imaginative"/"out of touch" the NN is) of the GTP
    #[arg(short, long, default_value_t = 0.7)]
    temperature: f64,

    ///
    #[arg(short, long)]
    cli: Option<String>,

    ///
    #[arg(short, long)]
    model: Option<Model>,
}

const API_KEY_ENV_NAME: &str = "OPENAI_API_KEY";
fn main() -> serde_json::Result<()> {
    let args = Args::parse();
    let (question, temperature, cli, key, model) = (
        args.question,
        args.temperature,
        args.cli,
        args.key,
        args.model,
    );

    if question.is_none() && cli.is_none() {
        panic!("Provide either --question or --cli argument to the program")
    }
    let api_key: String = match key {
        Some(k) => k,
        None => env::var(API_KEY_ENV_NAME)
        .expect("OpenAI key was not provided neither as an argument, nor as an enviroment variable OPENAI_API_KEY")
    };

    // Currently, only two modes of work w/ priority to the cli command
    // TODO: make some state machine
    let response = match cli {
        Some(description) => ask_cli(
            api_key,
            Some(Model::Gpt3_5Turbo),
            description,
            Some(temperature),
        ),
        None => ask_question_sync(api_key, model, question.unwrap(), Some(temperature)),
    };

    let successfull = match response {
        Ok(res) => res,
        Err(e) => match e {
            AskError::ParsingError(err) => {
                println!("Parsing error: {}", err);
                panic!("<Parsing error>");
            }
            _ => {
                dbg!("Uncaught error: {}", e);
                panic!("<Default error>");
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
