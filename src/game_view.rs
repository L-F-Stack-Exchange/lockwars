//! Handles game rendering.

use crate::Game;
use anyhow::{anyhow, Context as AnyhowContext, Result};
use graphics::line;
use graphics::rectangle;
use graphics::types::Color;
use graphics::{Context, Graphics};

/// The game view.
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
        use graphics::color::TRANSPARENT;
        use std::convert::TryFrom;

        // calculate layout
        let settings = &self.settings;
        let [view_width, view_height] = context.get_view_size();

        let center_x = view_width * 0.5;
        let center_y = view_height * 0.5;

        let n_columns = game.settings().n_columns;
        let n_rows = game.settings().n_rows;
        let n_total_columns = n_columns * 2;

        let n_columns_f64: f64 = u32::try_from(n_columns)
            .context("cannot calculate cell size")?
            .into();
        let n_rows_f64: f64 = u32::try_from(n_rows)
            .context("cannot calculate cell size")?
            .into();
        let n_total_columns_f64 = n_columns_f64 * 2.0;

        let max_cell_width = view_width * settings.game_area_percentage / n_total_columns_f64;
        let max_cell_height = view_height * settings.game_area_percentage / n_rows_f64;
        let cell_size = max_cell_width.min(max_cell_height);

        let game_area_width = cell_size * n_total_columns_f64;
        let game_area_height = cell_size * n_rows_f64;

        let game_area = rectangle::centered([
            center_x,
            center_y,
            game_area_width * 0.5,
            game_area_height * 0.5,
        ]);
        let game_area_left_x = center_x - game_area_width * 0.5;
        let game_area_right_x = center_x + game_area_width * 0.5;
        let game_area_top_y = center_y - game_area_height * 0.5;
        let game_area_bottom_y = center_y + game_area_height * 0.5;

        // draw background
        graphics::clear(settings.background_color, g);

        // draw vertical cell separators
        for pos in (1..n_columns).chain((n_columns + 1)..n_total_columns) {
            let pos: f64 = u32::try_from(pos)
                .context(anyhow!("cannot draw cell separators"))?
                .into();
            let x = center_x + (pos - n_columns_f64) * cell_size;

            settings.cell_separator.draw_from_to(
                [x, game_area_top_y],
                [x, game_area_bottom_y],
                &context.draw_state,
                context.transform,
                g,
            );
        }

        // draw horizontal cell separators
        for pos in 1..n_rows {
            let pos: f64 = u32::try_from(pos)
                .context(anyhow!("cannot draw cell separators"))?
                .into();
            let y = game_area_top_y + pos * cell_size;

            settings.cell_separator.draw_from_to(
                [game_area_left_x, y],
                [game_area_right_x, y],
                &context.draw_state,
                context.transform,
                g,
            );
        }

        // draw the division line
        settings.division_line.draw_from_to(
            [center_x, game_area_top_y],
            [center_x, game_area_bottom_y],
            &context.draw_state,
            context.transform,
            g,
        );

        // draw base border
        let border = rectangle::Rectangle::new(TRANSPARENT).border(settings.base_border);

        let base_start: f64 = u32::try_from(game.settings().base_span.start)
            .context(anyhow!("cannot draw base border"))?
            .into();
        let base_end: f64 = u32::try_from(game.settings().base_span.end)
            .context(anyhow!("cannot draw base border"))?
            .into();

        let left_base = rectangle::rectangle_by_corners(
            game_area_left_x,
            game_area_top_y + cell_size * base_start,
            game_area_left_x + cell_size,
            game_area_top_y + cell_size * base_end,
        );
        border.draw(left_base, &context.draw_state, context.transform, g);

        let right_base = rectangle::rectangle_by_corners(
            game_area_right_x - cell_size,
            game_area_top_y + cell_size * base_start,
            game_area_right_x,
            game_area_top_y + cell_size * base_end,
        );
        border.draw(right_base, &context.draw_state, context.transform, g);

        // draw the border of the game area
        let border = rectangle::Rectangle::new(TRANSPARENT).border(settings.game_area_border);
        border.draw(game_area, &context.draw_state, context.transform, g);

        Ok(())
    }
}

#[derive(Clone)]
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

    /// The border of the game area.
    pub game_area_border: rectangle::Border,

    /// The [division line].
    ///
    /// [division line]: ../game/index.html#division-line
    pub division_line: line::Line,

    /// The cell separators.
    pub cell_separator: line::Line,

    /// The border of the bases.
    pub base_border: rectangle::Border,
}

/// Checks that the argument is within the range [0.0, 1.0].
fn check_percentage(number: f64) -> Result<()> {
    if number >= 0.0 && number <= 1.0 {
        Ok(())
    } else {
        Err(anyhow!("{} is not a valid percentage", number))
    }
}
