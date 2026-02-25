use std::env;
use action::Action;

mod action;

fn main() {
    let args: Vec<String> = env::args().collect();

    match Action::parse(&args) {
        Ok(action) => action.execute(),
        Err(e) => eprintln!("forge: {}", e),
    }
}
