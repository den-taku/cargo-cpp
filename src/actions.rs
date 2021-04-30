use anyhow::Result;

pub fn when_new(name: String) -> Result<()> {
    let pb = indicatif::ProgressBar::new(12);

    // first, making directories
    crate::file_handler::do_mkdir(&name)?;
    pb.println(format!("[+] finished #{}", 1));
    pb.inc(1);

    crate::file_handler::do_mkdir(&format!("{}/main", &name))?;
    pb.println(format!("[+] finished #{}", 2));
    pb.inc(1);

    crate::file_handler::do_mkdir(&format!("{}/lib", &name))?;
    pb.println(format!("[+] finished #{}", 3));
    pb.inc(1);

    // next, making setting files for Bazel and Git
    crate::file_handler::write_file(crate::template::workspace(), format!("{}/WORKSPACE", &name))?;
    pb.println(format!("[+] finished #{}", 4));
    pb.inc(1);

    crate::file_handler::write_file(
        crate::template::git_ignore(),
        format!("{}/.gitignore", &name),
    )?;
    pb.println(format!("[+] finished #{}", 5));
    pb.inc(1);

    crate::file_handler::write_file(
        crate::template::build_main(name.clone()),
        format!("{}/main/BUILD", &name),
    )?;
    pb.println(format!("[+] finished #{}", 6));
    pb.inc(1);

    crate::file_handler::write_file(crate::template::build_lib(), format!("{}/lib/BUILD", &name))?;
    pb.println(format!("[+] finished #{}", 7));
    pb.inc(1);

    crate::file_handler::write_file(crate::template::main_cpp(&name), format!("{}/main/main.cpp", &name))?;
    pb.println(format!("[+] finished #{}", 8));
    pb.inc(1);

    crate::file_handler::write_file(crate::template::name_cpp(), format!("{}/main/{}.cpp", &name, &name))?;
    pb.println(format!("[+] finished #{}", 9));
    pb.inc(1);

    crate::file_handler::write_file(crate::template::name_h(), format!("{}/main/{}.h", &name, &name))?;
    pb.println(format!("[+] finished #{}", 10));
    pb.inc(1);

    crate::file_handler::write_file(crate::template::proconlib_cpp(), format!("{}/lib/proconlib.cpp", &name))?;
    pb.println(format!("[+] finished #{}", 11));
    pb.inc(1);

    crate::file_handler::write_file(crate::template::proconlib_h(), format!("{}/main/proconlib.h", &name))?;
    pb.println(format!("[+] finished #{}", 12));
    pb.inc(1);

    pb.finish_with_message("done");

    Ok(())
}

pub fn when_build() -> Result<()> {
    unimplemented!()
}

pub fn when_run() -> Result<()> {
    unimplemented!()
}

pub fn when_test() -> Result<()> {
    unimplemented!()
}

pub fn when_add() -> Result<()> {
    unimplemented!()
}

pub fn when_query() -> Result<()> {
    unimplemented!()
}
