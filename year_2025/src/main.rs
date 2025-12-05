use std::process::Command;

const HELP_MESSAGE: &str = r#"Usage: cargo solve <day as NN>
Arguments:
    <day as NN>    The day to solve, formatted as two digits (e.g., 01, 02, ..., 25)
"#;

fn print_error(msg: &str) {
    println!("{}\n{}", msg, HELP_MESSAGE);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        print_error("Too few arguments.");
        return;
    }
    if args.len() > 3 {
        print_error("Too many arguments.");
        return;
    }
    if args[1] == "solve" {
        let day_file = format!("src/bin/{}/main.rs", args[2]);
        if !std::path::Path::new(&day_file).exists() {
            println!("The specified day {} does not exist.", args[2]);

            let bin_path = std::path::Path::new("src/bin");
            if bin_path.exists() {
                if let Ok(entries) = std::fs::read_dir(bin_path) {
                    let valid_days: Vec<String> = entries
                        .filter_map(|entry| entry.ok())
                        .filter(|entry| entry.path().is_dir())
                        .filter(|entry| entry.path().join("main.rs").exists())
                        .filter_map(|entry| entry.file_name().to_str().map(String::from))
                        .collect();

                    if !valid_days.is_empty() {
                        println!("Available days: {}", valid_days.join(", "));
                    }
                }
            }

            return;
        }

        let output = Command::new("cargo")
            .args(&["run", "--bin"])
            .arg(&args[2])
            .output()
            .expect("Failed to execute command");

        println!("{}", String::from_utf8_lossy(&output.stdout));
        return;
    }

    print_error(&format!("Invalid argument \"{}\".", args[1]));
}
