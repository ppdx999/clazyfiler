mod actions;
mod app;
mod key;
mod modes;
mod terminal;
mod state;
mod ui;

use crate::{app::run_app, terminal::with_terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = with_terminal(run_app);
    
    if let Err(err) = result {
        println!("{:?}", err);
    }
    
    Ok(())
}

