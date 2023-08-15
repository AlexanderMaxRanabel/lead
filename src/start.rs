use std::process::Command;
use colored::*;

pub fn start_silverbullet(dirname: &str, mode: &str) {
    match mode {
        "expose" => {
            println!("{}", "Dont Forget an TLS Terminator!".red());
            let start = Command::new("silverbullet")
                .arg(dirname)
                .arg("-L")
                .arg("0.0.0.0")
                .output()
                .expect("Failed to start SilverBullet");

            if start.status.success() {
                println!("SilverBullet is now running: http://127.0.0.1:3000")
            } else {
                let stderr = String::from_utf8_lossy(&start.stderr);
                println!("{}", stderr);
            }
        },

        "normal" => {
            let start = Command::new("silverbullet")
                .arg(dirname)
                .output()
                .expect("Failed to start SilverBullet");

            if start.status.success() {
            } else {
                let stderr = String::from_utf8_lossy(&start.stderr);
                println!("{}", stderr);
            }
        },
        _ => println!("Unknown mode")
    }
}