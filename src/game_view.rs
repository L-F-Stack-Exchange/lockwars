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
    pub fn draw<G>(&self, game: &Game, context: &Context, g: &mut G) -> Result<()>
    where
        G: Graphics,
    {
        use graphics::rectangle::{self, Rectangle};
        use std::convert::TryFrom;

        let settings = &self.settings;
        let [view_width, view_height] = context.get_view_size();

        // calculate layout
        let n_total_columns: f64 = u32::try_from(game.settings().n_columns * 2)
            .context("cannot calculate cell size")?
            .into();
        let n_rows: f64 = u32::try_from(game.settings().n_rows)
            .context("cannot calculate cell size")?
            .into();

        let max_cell_width = view_width * settings.game_area_percentage / n_total_columns;
        let max_cell_height = view_height * settings.game_area_percentage / n_rows;
        let cell_size = max_cell_width.min(max_cell_height);

        let game_area_width = cell_size * n_total_columns;
        let game_area_height = cell_size * n_rows;

        let center_x = view_width * 0.5;
        let center_y = view_height * 0.5;

        // draw background
        graphics::clear(settings.background_color, g);

        // draw the border of the game area
        let game_area = rectangle::centered([
            center_x,
            center_y,
            game_area_width * 0.5,
            game_area_height * 0.5,
        ]);
        let border = Rectangle::new_border(
            settings.game_area_border_color,
            settings.game_area_border_thickness,
        );
        border.draw(game_area, &context.draw_state, context.transform, g);

        // draw the division line
        graphics::line_from_to(
            settings.division_line_color,
            settings.division_line_thickness,
            [center_x, center_y - game_area_height * 0.5],
            [center_x, center_y + game_area_height * 0.5],
            context.transform,
            g,
        );

        Ok(())
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
    /// with maximum width `window_width * game_area_percentage`
    /// and maximum height `window_height * game_area_percentage`
    /// located at the center of the screen.
    /// The actual game area might be smaller
    /// to preserve the proportion of cells.
    pub game_area_percentage: f64,

    /// The thickness of the border of the game area.
    pub game_area_border_thickness: f64,

    /// The color of the border of the game area.
    pub game_area_border_color: Color,

    /// The thickness of the [division line].
    ///
    /// [division line]: ../game/index.html#division-line
    pub division_line_thickness: f64,

    /// The color of the [division line].
    ///
    /// [division line]: ../game/index.html#division-line
    pub division_line_color: Color,
}

/// Checks that the argument is within the range [0.0, 1.0].
fn check_percentage(number: f64) -> Result<()> {
    if number >= 0.0 && number <= 1.0 {
        Ok(())
    } else {
        Err(anyhow!("{} is not a valid percentage", number))
    }
}
