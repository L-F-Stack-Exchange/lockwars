#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]

use anyhow::{anyhow, Result};
use glutin_window::GlutinWindow;
use graphics::color::{BLACK, WHITE};
use graphics::line;
use graphics::rectangle;
use lockwars::{
    game, game_controller, game_view, object, player, Cooldown, Game, GameController, GameView,
    Object, Player, Players,
};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    Button, ButtonEvent, EventSettings, Events, Key, RenderEvent, UpdateEvent, WindowSettings,
};
use std::time::Duration;

const WINDOW_TITLE: &str = "Lockwars";
const WINDOW_SIZE: (u32, u32) = (1280, 720);

fn main() -> Result<()> {
    let opengl = OpenGL::V3_2;

    let mut window = create_window(opengl)?;
    let mut gl = GlGraphics::new(opengl);

    let game = create_game()?;
    let mut game_controller = create_game_controller(game)?;
    let game_view = create_game_view()?;

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.button_args() {
            game_controller.button_event(args)?;
        }
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, g| {
                game_view.draw(&game_controller, &context, g)
            })?;
        }
        if let Some(args) = event.update_args() {
            game_controller.update_event(args)?;
        }
    }

    Ok(())
}

fn create_window(opengl: OpenGL) -> Result<GlutinWindow> {
    let window_settings = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(true)
        .decorated(true);
    GlutinWindow::new(&window_settings).map_err(|_| anyhow!("cannot create window"))
}

fn create_game() -> Result<Game> {
    use object::{Kind, Owned};

    let game_settings = game::Settings {
        n_columns: 6,
        n_rows: 7,
        base_span: 2..5,
        max_keys: 1000,
    };
    let players = Players {
        left: player_data(),
        right: player_data(),
    };

    game::Builder::new(game_settings)?
        .object(
            (3, 0),
            Owned {
                object: Object {
                    kind: Kind::Key {
                        generation: 10,
                        cooldown: Cooldown::new(Duration::from_secs(1)),
                    },
                    health: u32::MAX,
                    max_health: u32::MAX,
                },
                owner: Player::Left,
            },
        )?
        .object(
            (3, 11),
            Owned {
                object: Object {
                    kind: Kind::Key {
                        generation: 10,
                        cooldown: Cooldown::new(Duration::from_secs(1)),
                    },
                    health: u32::MAX,
                    max_health: u32::MAX,
                },
                owner: Player::Right,
            },
        )?
        .players(players)
        .finish()
}

fn player_data() -> player::Data {
    use object::Kind;
    use player::Placement;

    player::Data {
        keys: 200,
        placements: vec![
            Placement {
                cooldown: Cooldown::new(Duration::from_secs(1)),
                cost: 20,
                generate_object: Box::new(|| Object {
                    kind: Kind::Key {
                        generation: 10,
                        cooldown: Cooldown::new(Duration::from_secs(1)),
                    },
                    health: 100,
                    max_health: 100,
                }),
            },
            Placement {
                cooldown: Cooldown::new(Duration::from_secs(1)),
                cost: 40,
                generate_object: Box::new(|| Object {
                    kind: Kind::Fire {
                        damage: 20,
                        cooldown: Cooldown::new(Duration::from_secs(1)),
                    },
                    health: 100,
                    max_health: 100,
                }),
            },
            Placement {
                cooldown: Cooldown::new(Duration::from_secs(1)),
                cost: 20,
                generate_object: Box::new(|| Object {
                    kind: Kind::Barrier {},
                    health: 3600,
                    max_health: 3600,
                }),
            },
        ],
    }
}

fn create_game_controller(game: Game) -> Result<GameController> {
    use game_controller::KeyBinding;

    let game_controller_settings = game_controller::Settings {
        key_binding: Players {
            left: KeyBinding {
                up: Button::Keyboard(Key::W),
                down: Button::Keyboard(Key::S),
                left: Button::Keyboard(Key::A),
                right: Button::Keyboard(Key::D),
                remove: Button::Keyboard(Key::G),
                place: [Key::T, Key::Y, Key::U]
                    .iter()
                    .copied()
                    .map(Button::Keyboard)
                    .collect(),
            },
            right: KeyBinding {
                up: Button::Keyboard(Key::Up),
                down: Button::Keyboard(Key::Down),
                left: Button::Keyboard(Key::Left),
                right: Button::Keyboard(Key::Right),
                remove: Button::Keyboard(Key::NumPad0),
                place: [Key::NumPad1, Key::NumPad2, Key::NumPad3]
                    .iter()
                    .copied()
                    .map(Button::Keyboard)
                    .collect(),
            },
        },
        selected_cells: Players {
            left: (3, 0),
            right: (3, 11),
        },
    };

    GameController::new(game_controller_settings, game)
}

fn create_game_view() -> Result<GameView> {
    let game_view_settings = game_view::Settings {
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
        selected_cell_color: [0.0, 0.2, 0.0, 1.0],
        key_bar_border: rectangle::Border {
            color: WHITE,
            radius: 1.0,
        },
        key_bar_division_line: line::Line::new(WHITE, 1.0),
        key_bar_color: WHITE,
        health_bar_height_percentage: 0.7,
        health_bar_width_percentage: 0.04,
        health_bar_background: [0.4, 0.2, 0.2, 1.0],
        health_bar_color: [0.8, 0.4, 0.4, 1.0],
    };

    GameView::new(game_view_settings)
}
