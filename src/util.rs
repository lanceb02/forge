use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn dir_size(path: &Path) -> std::io::Result<u64> {
    let mut size = 0;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                size += metadata.len();
            } else if metadata.is_dir() {
                size += dir_size(&entry.path())?;
            }
        }
    }
    Ok(size)
}

pub fn yn_prompt(prompt: &str) -> bool {
    print!("{} [y/n]: ", prompt);
    io::stdout().flush().unwrap();

    // Read input from user
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Normalize input
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "y" | "yes" | "" => true,
        _ => false,
    }
}
