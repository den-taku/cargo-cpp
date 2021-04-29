use structopt::StructOpt;

/// Make new package for implement something in C++
#[derive(StructOpt)]
pub struct Cli {
    /// what want to do
    pub action: String,
}

