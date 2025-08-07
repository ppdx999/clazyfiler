mod app;
mod core;
mod handlers;
mod key;
mod messages;
mod model;
mod services;
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

