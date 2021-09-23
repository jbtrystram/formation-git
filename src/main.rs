mod sentences;

use crate::sentences::{Language, Sentences};

use clap::{AppSettings, Clap};

/// This program will help you with small talk when travelling.
#[derive(Clap)]
#[clap(version = "1.0", author = "JB Trystram")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
    #[clap(short, long, global = true, default_value = "English")]
    country: Language,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// Print a greeting message
    Hello,
    /// Use it when you need to leave
    Bye,
    /// ask where you can eat
    Food,
    /// Say thanks if someone was nice
    Thanks,
}

fn main() {
    let opts: Opts = Opts::parse();

    // instanciate the sentences
    let sentences = Sentences::new(opts.country);

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Hello => {
            println!("{}", sentences.hello);
        }
        SubCommand::Bye => {
            println!("{}", sentences.bye);
        }
        SubCommand::Food => {
            println!("{}", sentences.food);
        }
        SubCommand::Thanks => {
            println!("{}", sentences.thanks);
        }
    }
}
