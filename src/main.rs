use anyhow::{anyhow, Result};
use glutin_window::GlutinWindow;
use piston::{EventSettings, Events, WindowSettings};

const WINDOW_TITLE: &str = "Lockwars";
const WINDOW_SIZE: (u32, u32) = (960, 540);

fn main() -> Result<()> {
    let window_settings = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .resizable(true)
        .decorated(true);
    #[rustfmt::skip]
    let mut window = GlutinWindow::new(&window_settings)
        .map_err(|_| anyhow!("cannot create window"))?;

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    while let Some(_) = events.next(&mut window) {}

    Ok(())
}
