mod gui;
use gui::Gui;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut gui = Gui::new(1440, 700)?;
    gui.render();
    Ok(())
}
