//! Handles game rendering.

use crate::Game;
use anyhow::{anyhow, Context as AnyhowContext, Result};
use graphics::types::Color;
use graphics::{Context, Graphics};

/// The game view.
#[derive(Debug)]
pub struct GameView {
    settings: GameViewSettings,
}

impl GameView {
    /// Creates a new game view.
    pub fn new(settings: GameViewSettings) -> Result<Self> {
        check_percentage(settings.game_area_percentage).context("invalid margin percentage")?;

        Ok(Self { settings })
    }

    /// Draws the game on the screen.
    pub fn draw<G>(&self, _game: &Game, context: &Context, g: &mut G)
    where
        G: Graphics,
    {
        use graphics::rectangle::{self, Rectangle};

        let settings = &self.settings;
        let [view_width, view_height] = context.get_view_size();

        let game_area_width = view_width * settings.game_area_percentage;
        let game_area_height = view_height * settings.game_area_percentage;

        // draw background
        graphics::clear(settings.background_color, g);

        // draw the border of the game area
        let game_area = rectangle::centered([
            view_width * 0.5,
            view_height * 0.5,
            game_area_width * 0.5,
            game_area_height * 0.5,
        ]);
        let border = Rectangle::new_border(
            settings.game_area_border_color,
            settings.game_area_border_thickness,
        );
        border.draw(game_area, &context.draw_state, context.transform, g);
    }
}

#[derive(Clone, Debug)]
/// The game view settings.
pub struct GameViewSettings {
    /// The background color of the screen.
    pub background_color: Color,

    /// The percentage of the screen taken up by the game area,
    /// in both the horizontal and vertical directions.
    ///
    /// The game area is a rectangle
    /// with width `window_width * game_area_percentage`
    /// and height `window_height * game_area_percentage`
    /// located at the center of the screen.
    pub game_area_percentage: f64,

    /// The thickness of the border of the game area.
    pub game_area_border_thickness: f64,

    /// The color of the border of the game area.
    pub game_area_border_color: Color,
}

/// Checks that the argument is within the range [0.0, 1.0].
fn check_percentage(number: f64) -> Result<()> {
    if number >= 0.0 && number <= 1.0 {
        Ok(())
    } else {
        Err(anyhow!("{} is not a valid percentage", number))
    }
}
