use structopt::clap::arg_enum;
use structopt::{clap, StructOpt};

/// Make new package for implement something in C++
#[derive(StructOpt)]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Cli {
    /// what want to do
    pub action: Action,

    /// pacage's name
    pub name: Option<String>,

    /// add source as library
    #[structopt(short = "l", long = "library")]
    pub lib_name: Option<String>,
}

arg_enum! {
    #[derive(Debug)]
    pub enum Action {
        New,
        Build,
        Run,
        Clean,
        Test,
        Add,
        Query,
    }
}
