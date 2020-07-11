use anyhow::{anyhow, Context, Result};
use glutin_window::GlutinWindow;
use graphics::color::{BLACK, WHITE};
use lockwars::{Game, GameView, GameViewSettings};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventSettings, Events, RenderEvent, WindowSettings};

const WINDOW_TITLE: &str = "Lockwars";
const WINDOW_SIZE: (u32, u32) = (1280, 720);

fn main() -> Result<()> {
    let opengl = OpenGL::V3_2;

    let window_settings = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(true)
        .decorated(true);
    #[rustfmt::skip]
    let mut window = GlutinWindow::new(&window_settings)
        .map_err(|_| anyhow!("cannot create window"))?;

    let mut gl = GlGraphics::new(opengl);

    let game_view_settings = GameViewSettings {
        background_color: BLACK,
        game_area_percentage: 0.8,
        game_area_border_thickness: 1.0,
        game_area_border_color: WHITE,
    };

    let game = Game::new();
    let game_view = GameView::new(game_view_settings).context("cannot create game view")?;

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, g| {
                game_view.draw(&game, &context, g);
            });
        }
    }

    Ok(())
}
