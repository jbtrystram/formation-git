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
    /// ask where you can eat
    Food,
    /// Say thanks if someone was nice
    Thanks,
    /// Ask for a direction to the train station
    Station,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Some debugging
    // println!("Using subcommand : {:?}", opts.subcmd);

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Hello => {
            println!("Hello ! I am a traveller ! How are you ?");
        }
        SubCommand::Bye => {
            println!("See you later ! Bye bye !");
        }
        SubCommand::Food => {
            println!("I am hungry ! Is there a good restaurant around ?");
        }
        SubCommand::Thanks => {
            println!("Thanks you so much for helping me !");
        }
        SubCommand::Station => {
            println!("Can  me towards the train station");
        }
    }
}
