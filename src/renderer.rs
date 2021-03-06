//! Handles game rendering.

use crate::{object, Controller, Object, Player};
use anyhow::{anyhow, Context as AnyhowContext, Result};
use graphics::{line, math::Vec2d, rectangle, types::Color, Context, Graphics};

/// The game renderer.
pub struct Renderer {
    settings: Settings,
}

impl Renderer {
    /// Creates a new game renderer.
    pub fn new(settings: Settings) -> Result<Self> {
        check_percentage(settings.game_area_percentage).context("invalid game area percentage")?;
        check_percentage(settings.object_percentage).context("invalid object percentage")?;

        Ok(Self { settings })
    }

    /// Draws the game on the screen.
    #[allow(clippy::too_many_lines)]
    pub fn draw<G>(&self, game_controller: &Controller, context: &Context, g: &mut G) -> Result<()>
    where
        G: Graphics,
    {
        use graphics::color::TRANSPARENT;
        use std::convert::TryFrom;

        let game = game_controller.game();

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

        // draw selected cells
        for player in [Player::Left, Player::Right].iter().copied() {
            let (row, column) = game_controller.selected_cells()[player];

            let row: f64 = u32::try_from(row)
                .context("cannot draw selected cells")?
                .into();
            let column: f64 = u32::try_from(column)
                .context("cannot draw selected cells")?
                .into();

            rectangle::Rectangle::new(settings.selected_cell_color).draw(
                [
                    game_area_left_x + column * cell_size,
                    game_area_top_y + row * cell_size,
                    cell_size,
                    cell_size,
                ],
                &context.draw_state,
                context.transform,
                g,
            );
        }

        // draw objects
        for ((row, column), cell) in game.cells().indexed_iter() {
            let row: f64 = u32::try_from(row).context("cannot draw objects")?.into();
            let column: f64 = u32::try_from(column).context("cannot draw objects")?.into();

            let cell = cell.borrow();
            let object = match &cell.object {
                None => continue,
                Some(object) => &object.object,
            };

            let position = [
                game_area_left_x + column * cell_size,
                game_area_top_y + row * cell_size,
            ];
            self.draw_object(object, position, cell_size, context, g)?;
        }

        // draw vertical cell separators
        for pos in (1..n_columns).chain((n_columns + 1)..n_total_columns) {
            let pos: f64 = u32::try_from(pos)
                .context("cannot draw cell separators")?
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
                .context("cannot draw cell separators")?
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
            .context("cannot draw base border")?
            .into();
        let base_end: f64 = u32::try_from(game.settings().base_span.end)
            .context("cannot draw base border")?
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

        // draw the key bar
        let bottom_margin_height = view_height - game_area_bottom_y;
        let key_bar_height = bottom_margin_height / 4.0;

        let key_bar_top_y = game_area_bottom_y + bottom_margin_height / 4.0;
        let key_bar_bottom_y = game_area_bottom_y + bottom_margin_height / 2.0;

        let key_bar_area = rectangle::rectangle_by_corners(
            game_area_left_x,
            key_bar_top_y,
            game_area_right_x,
            key_bar_bottom_y,
        );

        let border = rectangle::Rectangle::new(TRANSPARENT).border(settings.key_bar_border);
        border.draw(key_bar_area, &context.draw_state, context.transform, g);

        // draw the key bar division line
        settings.key_bar_division_line.draw_from_to(
            [center_x, key_bar_top_y],
            [center_x, key_bar_bottom_y],
            &context.draw_state,
            context.transform,
            g,
        );

        // fill the key bar
        let key_bar_width = game_area_width / 2.0;
        let max_keys: f64 = game.settings().max_keys.into();

        for (player, offset) in [(Player::Left, 0.0), (Player::Right, key_bar_width)]
            .iter()
            .copied()
        {
            let filled_area = [
                game_area_left_x + offset,
                key_bar_top_y,
                f64::from(game.players()[player].keys) / max_keys * key_bar_width,
                key_bar_height,
            ];
            rectangle::Rectangle::new(settings.key_bar_color).draw(
                filled_area,
                &context.draw_state,
                context.transform,
                g,
            );
        }

        Ok(())
    }

    /// Draws the object at the specified position.
    pub fn draw_object<G>(
        &self,
        object: &Object,
        position: Vec2d,
        cell_size: f64,
        context: &Context,
        g: &mut G,
    ) -> Result<()>
    where
        G: Graphics,
    {
        use graphics::ellipse;
        use object::Kind;

        // calculate layout
        let settings = &self.settings;

        let cell_left_x = position[0];
        let cell_right_x = cell_left_x + cell_size;
        let cell_top_y = position[1];
        // let cell_bottom_y = cell_top_y + cell_size;

        let center_x = cell_left_x + cell_size * 0.5;
        let center_y = cell_top_y + cell_size * 0.5;

        let object_size = cell_size * settings.object_percentage;
        let object_area = rectangle::centered_square(center_x, center_y, object_size * 0.5);

        let object_left_x = center_x - object_size * 0.5;
        let object_right_x = center_x + object_size * 0.5;
        let object_top_y = center_y - object_size * 0.5;
        let object_bottom_y = center_y + object_size * 0.5;

        // draw object
        match object.kind {
            Kind::Key { .. } => {
                // draw regular triangle
                let offset = (1.0 - f64::sqrt(3.0) / 2.0) / 2.0 * object_size;
                let outline = [
                    [object_left_x, object_bottom_y - offset],
                    [object_right_x, object_bottom_y - offset],
                    [center_x, object_top_y + offset],
                ];

                let line = line::Line::new(
                    settings.object_outline_color,
                    settings.object_outline_radius,
                );
                draw_polygon_border(line, &outline, context, g);
            }
            Kind::Fire { .. } => {
                // draw circle
                let circle = ellipse::Ellipse::new_border(
                    settings.object_outline_color,
                    settings.object_outline_radius,
                );
                circle.draw(object_area, &context.draw_state, context.transform, g);
            }
            Kind::Barrier { .. } => {
                // draw square
                let rectangle = rectangle::Rectangle::new_border(
                    settings.object_outline_color,
                    settings.object_outline_radius,
                );
                rectangle.draw(object_area, &context.draw_state, context.transform, g);
            }
        }

        // draw health bar
        let health_bar_height = cell_size * settings.health_bar_height_percentage;
        let health_bar_width = cell_size * settings.health_bar_width_percentage;

        let health_bar_center_x = (object_right_x + cell_right_x) / 2.0;
        let health_bar_center_y = center_y;

        let health_bar_area = rectangle::centered([
            health_bar_center_x,
            health_bar_center_y,
            health_bar_width / 2.0,
            health_bar_height / 2.0,
        ]);

        let [health_bar_left_x, health_bar_top_y, ..] = health_bar_area;
        let health_bar_right_x = health_bar_left_x + health_bar_width;
        let health_bar_bottom_y = health_bar_top_y + health_bar_height;

        rectangle::Rectangle::new(settings.health_bar_background).draw(
            health_bar_area,
            &context.draw_state,
            context.transform,
            g,
        );

        // fill the health bar
        let filled_area = rectangle::rectangle_by_corners(
            health_bar_left_x,
            health_bar_bottom_y
                - f64::from(object.health) / f64::from(object.max_health) * health_bar_height,
            health_bar_right_x,
            health_bar_bottom_y,
        );
        rectangle::Rectangle::new(settings.health_bar_color).draw(
            filled_area,
            &context.draw_state,
            context.transform,
            g,
        );

        Ok(())
    }
}

#[derive(Clone)]
/// The game renderer settings.
pub struct Settings {
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

    /// The percentage of a cell taken up by the object within,
    /// in both the horizontal and vertical directions.
    pub object_percentage: f64,

    /// The color of outlines of objects
    pub object_outline_color: Color,

    /// The radius of outlines of objects
    pub object_outline_radius: f64,

    /// The color of cells selected by players.
    pub selected_cell_color: Color,

    /// The border of the key bar.
    pub key_bar_border: rectangle::Border,

    /// The line that separates the players' areas in the key bar.
    pub key_bar_division_line: line::Line,

    /// The color to fill the key bar.
    pub key_bar_color: Color,

    /// The height of the health bar,
    /// as a percentage of the cell size.
    pub health_bar_height_percentage: f64,

    /// The height of the health bar,
    /// as a percentage of the cell size.
    pub health_bar_width_percentage: f64,

    /// The background color of the health bar.
    pub health_bar_background: Color,

    /// The color to fill the health bar.
    pub health_bar_color: Color,
}

/// Checks that the argument is within the range [0.0, 1.0].
fn check_percentage(number: f64) -> Result<()> {
    if number >= 0.0 && number <= 1.0 {
        Ok(())
    } else {
        Err(anyhow!("{} is not a valid percentage", number))
    }
}

/// Draws the specified polygon border.
fn draw_polygon_border<G>(
    line: line::Line,
    polygon: graphics::types::Polygon,
    context: &Context,
    g: &mut G,
) where
    G: Graphics,
{
    use itertools::Itertools;

    for (from, to) in polygon
        .iter()
        .copied()
        .cycle()
        .tuple_windows()
        .take(polygon.len())
    {
        line.draw_from_to(from, to, &context.draw_state, context.transform, g);
    }
}
