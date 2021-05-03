// use human_panic::setup_panic;
use anyhow::Result;
use cli::Action::*;
use structopt::StructOpt;

mod actions;
mod cli;
mod file_handler;
mod template;

fn main() -> Result<()> {
    // setup_panic!();

    let args = cli::Cli::from_args();

    match args.action {
        New => actions::when_new(args.name.unwrap())?,
        Build => actions::when_build()?,
        Run => actions::when_run()?,
        Clean => actions::when_clean()?,
        Test => actions::when_test()?, // unimplemented
        Add => actions::when_add()?,   // unimplemented
        Query => actions::when_query()?,
        Fmt => actions::when_fmt()?,
    }

    /*
    match args.action {
        New => {
            println!("args: new\nMake files and directories...\n");
            println!(
                "./main/main.cpp:\n{}",
                template::main_cpp(&args.name.clone().unwrap())
            );
            println!(
                "./main/BUILD:\n{}",
                template::build_main(args.name.clone().unwrap())
            );
            println!("./lib/BUILD:\n{}", template::build_lib());
            println!("./WORKSPACE:\n{}", template::workspace());
            println!("./.gitignore:\n{}", template::git_ignore());
            println!("./config:\n{}", template::config(args.name.unwrap()));
        }
        Build => {
            println!("args: build");
            println!("READ config");
            println!("$ bazel build //main:{}", "filename");
        }
        Run => {
            println!("args: run");
            println!("READ config");
            println!("$ bazel run //main:{}", "filename");
        }
        Test => {
            println!("args: test");
            println!("READ config");
            println!("TEST");
        }
        Add => {
            // For this, dentaku has to know more about Bazel
            println!("args: add");
            if let Some(name) = args.lib_name {
                println!("add statement to BUILD");
                println!("cc_library(\n    name = \"{}\",\n    srcs = [\"{}.cpp\"]\n    hdrs = [\"{}\".h],\n)]", name, name, name);
                println!("add statement to cc_binary in BUILD");
                println!("    deps = [\":{}.h\",],", name);
            } else {
                // happend?
                println!("add statement to cc_binary in BUILD");
                println!("")
            }
            println!("add sentence to cc_library in BUILD:");
            println!(" hdrs = [\"filename.h\"] in cc_binary");
        }
        Query => {
            println!("args: query");
            println!("READ config");
            println!(
                "$ bazel query --nohost_deps --noimplicit_deps 'deps(//main:{})' --output graph",
                "name"
            );
        }
    }
    // */
    Ok(())
}
