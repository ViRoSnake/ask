use clap::Parser;
use serde::{Deserialize, Serialize};
use ask::{ask_one, AskError, Model};
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
