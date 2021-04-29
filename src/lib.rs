use structopt::StructOpt;

/// Make new package for implement something in C++
#[derive(StructOpt)]
pub struct Cli {
    /// what want to do
    pub action: String,
}

pub fn main_cpp() -> String {
    "#include <iostream>\n// using namespace std;\n\nint main() {\n    //\n}\n".to_string()
}
