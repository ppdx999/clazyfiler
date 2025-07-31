mod actions;
mod app;
mod modes;
mod terminal;
mod ui;

use crate::{app::run_app, terminal::with_terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = with_terminal(run_app);
    
    if let Err(err) = result {
        println!("{:?}", err);
    }
    
    Ok(())
}

