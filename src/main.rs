use anyhow::{anyhow, Context, Result};
use glutin_window::GlutinWindow;
use graphics::color::{BLACK, WHITE};
use graphics::line;
use graphics::rectangle;
use lockwars::{
    Cell, Game, GameSettings, GameView, GameViewSettings, Object, ObjectKind, PlayerData, Players,
};
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

    let game_settings = GameSettings {
        n_columns: 6,
        n_rows: 7,
        base_span: 2..5,
    };
    let players = Players {
        left: PlayerData {
            selected_position: (3, 0),
        },
        right: PlayerData {
            selected_position: (3, 11),
        },
    };
    let mut game = Game::new(game_settings, players).context("cannot create game")?;

    let mut cells = game.cells_mut();
    cells[(3, 0)] = Cell {
        object: Some(Object {
            kind: ObjectKind::Key,
        }),
    };
    cells[(3, 11)] = Cell {
        object: Some(Object {
            kind: ObjectKind::Fire,
        }),
    };

    let game_view_settings = GameViewSettings {
        background_color: BLACK,
        game_area_percentage: 0.8,
        game_area_border: rectangle::Border {
            color: WHITE,
            radius: 1.0,
        },
        division_line: line::Line::new(WHITE, 1.0),
        cell_separator: line::Line::new([0.5, 0.5, 0.5, 1.0], 1.0),
        base_border: rectangle::Border {
            color: WHITE,
            radius: 1.0,
        },
        object_percentage: 0.6,
        object_outline_color: [0.8, 0.4, 0.4, 1.0],
        object_outline_radius: 1.0,
    };
    let game_view = GameView::new(game_view_settings).context("cannot create game view")?;

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, g| {
                game_view.draw(&game, &context, g)
            })?;
        }
    }

    Ok(())
}
