/// This is a template for ./main/main.cpp
pub fn main_cpp(name: &str) -> String {
    format!("#include <iostream>\n#include \"{}.h\"\n#include \"lib/proconlib.h\"\n// using namespace std;\n\nint main() {{\n    std::cout << \"Hello, world\" << std::endl;\n    return 0;\n}}\n", &name)
}

/// This is a template for ./main/BUILD
pub fn build_main(name: String) -> String {
    format!(
        "cc_library (\n    name = \"{}\",\n    srcs = [\"{}.cpp\"],\n    hdrs = [\"{}.h\"],\n)\n\ncc_binary (\n    name = \"main\",\n    srcs = [\"main.cpp\"],\n    deps = [\n        \":{}\",\n        \"//lib:proconlib\",\n    ],\n)\n",
        &name,
        &name,
        &name,
        name
    )
}

/// This is a template for ./lib/BUILD
pub fn build_lib() -> String {
    "cc_library (\n    name = \"proconlib\",\n    srcs = [\"proconlib.cpp\"],\n    hdrs = [\"proconlib.h\"],\n    visibility = [\"//main:__pkg__\"],\n)\n".to_string()
}

/// This is a template for ./WORKSPACE
pub fn workspace() -> String {
    "\n".to_string()
}

/// This is a template for ./.gitignore
pub fn git_ignore() -> String {
    "/bazel-*\n".to_string()
}

pub fn config(name: String) -> String {
    format!("name = \"{}\"\nlibrary = false", name)
}

pub fn name_cpp() -> String {
    "// Let's start coding!\n// Coder: DenTaku\n\nvoid somefunc() {\n    //\n}\n".to_string()
}

pub fn name_h() -> String {
    "// write signiture as prototype declaration.\n\nvoid somefunc();\n".to_string()
}

pub fn proconlib_cpp() -> String {
    "// for libraries ...\n// I'll add codes in the future...\n\nvoid somelib() {\n    //\n}\n".to_string()
}

pub fn proconlib_h() -> String {
    "// prototype declarations will be written here...\n\nvoid somelib();\n".to_string()
}