use std::process::{Command, Stdio};

use which::which;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // check if gun exists, if not: panic!
    which("gum").expect("gum not found. Please install gum first.");

    let gum_proc = Command::new("gum")
        .args(["choose", "apple", "banana", "cherry"])
        .stdout(Stdio::piped())
        .spawn()?;

    let gum_output = gum_proc.wait_with_output()?;
    let gum_choice = String::from_utf8_lossy(&gum_output.stdout)
        .trim()
        .to_string();
    println!("selected choice: {}", gum_choice);

    Ok(())
}
