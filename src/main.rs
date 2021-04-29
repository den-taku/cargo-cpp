use human_panic::setup_panic;
use structopt::StructOpt;

fn main() {
    setup_panic!();

    let args = cargo_cpp::Cli::from_args();

    println!("{}", args.action);
    println!("{}", cargo_cpp::main_cpp());

}
