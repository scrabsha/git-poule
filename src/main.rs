use std::env;
use std::process::Command;

fn main() {
    let args = env::args_os().skip(2);

    Command::new("git")
        .arg("pull")
        .args(args)
        .spawn()
        .expect("Failed to run git pull")
        .wait()
        .expect("Failed to wait child")
        .code();
}
