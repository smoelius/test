use snapbox::cmd::OutputAssert;
use std::env::{args, current_exe};
use std::process::Command;

fn main() {
    let path = current_exe().unwrap();
    if args().len() <= 1 {
        println!("{}", path.display());
        eprintln!("{}", path.display());
    } else {
        let assert = OutputAssert::new(Command::new(&path).output().unwrap());
        assert
            .stdout_eq(format!("{}\n", path.to_str().unwrap()))
            .stderr_eq(format!("{}\n", path.to_str().unwrap()));
    }
}
