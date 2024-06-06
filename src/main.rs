use std::{env, process::{exit, Command}};

fn main() {
    let version = env::args().nth(1).expect("PHP version is not selected");

    let available_versions = [String::from("74"), String::from("81"), String::from("82"), String::from("83")];
    if !available_versions.contains(&version) {
        println!("Version {version} is not supported");
        exit(1);
    }

    let amount = env::args().count();

    let mut args = vec![String::new(); amount - 2];

    let mut index = 0;
    for arg in env::args() {
        index += 1;
        if index <= 2 {
            continue;
        }

        args[index - 3] = arg;
    }

    let mut command = "C:/xampp/php".to_owned();
    command.push_str(&version);
    command.push_str("/php.exe");

    let mut process = Command::new(command)
        .args(args)
        .spawn()
        .expect("Multi PHP: Failed to execute command");

    let status = process.wait().unwrap();
    println!("Exited with {status}")

}
