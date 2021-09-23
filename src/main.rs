use clap::{AppSettings, Clap};

/// This program will help you with small talk when travelling.
#[derive(Clap)]
#[clap(version = "1.0", author = "JB Trystram")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// Print a greeting message
    Hello,
    /// Use it when you need to leave
    Bye,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Some debugging
    // println!("Using subcommand : {:?}", opts.subcmd);

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Hello => {
            println!("Hello ! How are you ?");
        }
        SubCommand::Bye => {
            println!("See you later ! Bye bye !");
        }
    }
}
