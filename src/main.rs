mod start;
use start::*;

use std::env;
use std::io::{self, Write};
use std::fs::File;

use colored::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let supermode = &args[1].to_string();
    } else {
        println!("{}", "Welcome to Lead Interactive Shell".magenta());
        let mut buffer = io::stdout();
        loop {
            buffer.write_all(b"> ")?;
            buffer.flush()?;
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect("Failed to read input");
            let tokens: Vec<&str> = command.split_whitespace().collect();
            match tokens[0] {
                "start" => {
                    if tokens.len() > 3 {
                        println!("Error: Over Satisfied");
                    } else {
                        start_silverbullet(tokens[1], tokens[2]);
                        println!("SilverBullet is now running: http://127.0.0.1:3000")
                    }
                },

                "new" => {
                    match tokens[1]  {
                        "file" =>  {
                            let _file = File::create(tokens[2])?;
                        }
                        _ => println!("Usage: new [1]")
                    }
                },

                "instant" => {
                    let index_md = format!(r#"
                    # Welcome To {} Silver Bullet Space!
                    ## Some Common Keybinds:
                    - Alt+Shift+N = Creates a new page!
                    "#, tokens[1]);

                    let settings_md = format!(r#"
                    This page contains settings for configuring SilverBullet and its plugs. Any changes outside of the yaml block will be overwritten.

```yaml
indexPage: index
```
                    "#);
                    let indexdotmd_path = tokens[1].to_owned() + "/" + "index.md";
                    let settingsdotmd_path = tokens[1].to_owned() + "/" + "SETTINGS.md";

                    let mut indexdotmd = File::create(indexdotmd_path)?;
                    let mut settingsdotmd = File::create(settingsdotmd_path)?;

                    indexdotmd.write_all(index_md.trim().as_bytes())?;
                    settingsdotmd.write_all(settings_md.trim().as_bytes())?;
                },

                "exit" => {
                    std::process::exit(0);
                },
                _ => println!("Unknown Command")
            }
        }
    }
    Ok(())
}
