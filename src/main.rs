use clap::Parser;

use ask::{ask_one, ask_cli, AskError, Model};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Question to ask GTP
    #[arg(short, long)]
    question: Option<String>,

    /// Temperature (how "imaginative"/"out of touch" the NN is) of the GTP
    #[arg(short, long, default_value_t = 0.7)]
    temperature: f64,

    ///
    #[arg(short, long)]
    cli: Option<String>,
}

fn main() -> serde_json::Result<()> {
    let args = Args::parse();
    let (question, temperature, cli) = (args.question, args.temperature, args.cli);

    if question.is_none() && cli.is_none() {
        panic!("Provide either --question or --cli argument to the program")
    }

    // Currently, only two modes of work w/ priority to the cli command
    // TODO: make some state machine
    let response = match cli {
        Some(description) => ask_cli(Some(Model::Gpt3_5Turbo), description, Some(temperature)),
        None => ask_one(Some(Model::Gpt3_5Turbo), question.unwrap(), Some(temperature)),
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
