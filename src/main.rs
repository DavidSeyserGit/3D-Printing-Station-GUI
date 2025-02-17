mod gui;
use gui::Gui;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let mut gui = Gui::new(600, 400)?;
    gui.render();
    println!("Hello, world!");
    Ok(())
}
