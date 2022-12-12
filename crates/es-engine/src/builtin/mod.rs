use std::env;
use std::io;
use std::path::Path;
use std::process;


pub fn exit(code: i32) {
    process::exit(code)
}

pub fn abort() {
    process::abort()
}

pub fn cd(string: &str) -> io::Result<()> {
    let path = Path::new(string);

    env::set_current_dir(&path)?;

    env::set_var("PWD", path);

    Ok(())
}

// pub fn open(){}

// pub fn close()