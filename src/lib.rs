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
    pub lib_name: Option<String>
}

arg_enum! {
    #[derive(Debug)]
    pub enum Action {
        new,
        build,
        run,
        test,
        add,
        query,
    }
}

/// This is a template for ./main/main.cpp
pub fn main_cpp() -> String {
    "#include <iostream>\n// using namespace std;\n\nint main() {\n    std::cout << \"Hello, world\" << std::endl;\n    return 0;\n}\n".to_string()
}

/// This is a template for ./main/BUILD
pub fn build(name: String) -> String {
    format!(
        "cc_binary (\n    name = \"{}\",\n    srcs = [\"{}.cpp\"]\n)",
        name.clone(),
        name
    )
}

/// This is a template for ./WORKSPACE
pub fn workspace() -> String {
    "".to_string()
}

pub fn config(name: String) -> String {
    format!("name = \"{}\"\nlibrary = false", name)
}

pub fn git_ignore() -> String {
    "/bazel-*\n".to_string()
}
