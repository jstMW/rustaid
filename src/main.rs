pub mod ascii_art;
use colored::*;
use structopt::{self, StructOpt};

fn main() {
    ///this is a rustaid (rust + said) program like cowsay

    #[derive(StructOpt)]
    struct Options {
        #[structopt(default_value = "This is default message used when no message specified!")]
        ///give a one word message
        message: String,

        #[structopt(short = "d", long = "dead")]
        ///make the ferrise dead
        is_dead: bool,

        #[structopt(short = "c", long = "color")]
        text_color: String,
    }

    let option: Options = Options::from_args();
    let message = option.message;

    if message.to_lowercase() == "javascript" {
        eprintln!("go fuck yourself");
    } else {
        match &option.text_color[..] {
            "blue" => match option.is_dead {
                true => ascii_art::print_rust_dead(&message[..].blue()),
                false => ascii_art::print_rust_alive(&message[..].blue()),
            },
            _ => (),
        };
    }
}
