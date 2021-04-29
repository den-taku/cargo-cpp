// use human_panic::setup_panic;
use anyhow::Result;
use structopt::StructOpt;
use cargo_cpp::Action::*;

fn main() -> Result<()> {
    // setup_panic!();

    let args = cargo_cpp::Cli::from_args();

    match args.action {
        new => {
            println!("args: new\nMake files and directories...\n");
            println!("./main/main.cpp:\n{}", cargo_cpp::main_cpp());
            println!("./main/BUILD:\n{}", cargo_cpp::build(args.name.clone().unwrap()));
            println!("./WORKSPACE:\n{}", cargo_cpp::workspace());
            println!("./config:\n{}", cargo_cpp::config(args.name.unwrap()));
        },
        build => {
            println!("args: build");
            println!("READ config");
            println!("$ bazel build //main:{}", "filename");
        },
        run => {
            println!("args: run");
            println!("READ config");
            println!("$ bazel run //main:{}", "filename");
        },
        test => {
            println!("args: test");
            println!("READ config");
            println!("TEST");
        },
        add => {
            println!("args: add");
            if let Some(name) = args.lib_name {
                println!("add statement to BUILD");
                println!("cc_library(\n    name = \"{}\",\n    srcs = [\"{}.cpp\"]\n    hdrs = [\"{}\".h],\n)]", name, name, name);
                println!("add statement to cc_binary in BUILD");
                println!("    deps = [\":{}.h\",],", name);
            } else { // happend?
                println!("add statement to cc_binary in BUILD");
                println!("")
            }
            println!("add sentence to cc_library in BUILD:");
            println!(" hdrs = [\"filename.h\"] in cc_binary");
        }
    }

    

    Ok(())
}
