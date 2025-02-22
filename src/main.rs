use snapbox::cmd::OutputAssert;
use std::env::{args, current_exe};
use std::process::Command;

fn main() {
    let path = current_exe().unwrap();
    if args().len() <= 1 {
        println!("{}", path.display());
        eprintln!("{}", path.display());
        return;
    }

    let arg = args().skip(1).next().unwrap();

    if arg == "stdout" {
        let assert = OutputAssert::new(Command::new(&path).output().unwrap());
        assert.stdout_eq(format!("{}\n", path.to_str().unwrap()));
        return;
    }

    if arg == "stderr" {
        let assert = OutputAssert::new(Command::new(&path).output().unwrap());
        assert.stderr_eq(format!("{}\n", path.to_str().unwrap()));
        return;
    }

    panic!("expected `stdout` or `stderr`: {}", arg);
}
