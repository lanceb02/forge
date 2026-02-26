use action::Action;
use std::env;

mod action;
mod util;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match Action::parse(&args) {
        Ok(action) => {
            if let Err(e) = action.execute() {
                eprintln!("forge: {}", e);
            }
        }
        Err(e) => eprintln!("forge: {}", e),
    }
    Ok(())
}
