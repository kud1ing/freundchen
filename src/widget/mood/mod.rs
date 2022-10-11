mod month_labels;
mod mood_value_labels;

use crate::widget::mood::month_labels::MonthLabels;
use crate::widget::mood::mood_value_labels::MoodValueLabels;
use crate::{Piet, Region, Size};
use chrono::{Datelike, Local};
use guiver::widget::{WidgetCommand, WidgetError};
use guiver::{
    Color, Error, Event, Font, Line, PaintBrush, Point, Rect, RenderContext, SizeConstraints,
    Stroke, Widget, WidgetCore, WidgetEvent, WidgetId,
};
use std::collections::HashSet;
use std::usize;

/// The mood value datatype.
type MoodValue = u8;

/// The number of mood values per day.
const NUMBER_OF_MOOD_VALUES_PER_DAY: MoodValue = 11;

///
#[derive(Clone, Debug)]
pub struct MoodValuesUpdate {
    pub day_of_month_index: u8,
    pub mood_values: HashSet<MoodValue>,
}

// =================================================================================================

/// A widget that displays mood values for each day of a month.
pub struct MoodWidget {
    core: WidgetCore,
    day_of_month_index_today: usize,
    fills_per_mood_value_index: [PaintBrush; NUMBER_OF_MOOD_VALUES_PER_DAY as usize],
    month_labels: MonthLabels,
    month_labels_spacing: f64,
    mood_cell_size: Size,
    mood_cells_area_rectangle: Rect,
    mood_value_labels: MoodValueLabels,
    mood_values_per_day_of_month_index: Vec<HashSet<MoodValue>>,
    number_of_days_in_month: u8,
    mood_cells_grid_stroke: Stroke,
}

impl MoodWidget {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke) -> Self {
        // TODO: Use 31 or adjust to the current month?
        let number_of_days_in_month = 31;

        // Determine the day of month.
        let day_of_month_index_today = Local::today().naive_local().day0() as usize;

        let grid_color = Color::rgb8(100, 100, 100);

        let mut month_labels_font = Font::default();
        month_labels_font.font_color = grid_color.clone();

        let mood_value_labels_font = Font::default();

        let month_labels_spacing = 20.0;

        let mut mood_value_per_day_of_month_index = vec![];

        // Create the mood values per day.
        for _ in 0..number_of_days_in_month {
            mood_value_per_day_of_month_index.push(HashSet::new());
        }

        MoodWidget {
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            day_of_month_index_today,
            fills_per_mood_value_index: [
                // Darkest.
                PaintBrush::Color(Color::rgb8(55, 6, 23)),
                //
                PaintBrush::Color(Color::rgb8(86, 1, 29)),
                PaintBrush::Color(Color::rgb8(118, 0, 32)),
                PaintBrush::Color(Color::rgb8(149, 0, 30)),
                PaintBrush::Color(Color::rgb8(180, 0, 22)),
                //
                // Middle.
                PaintBrush::Color(Color::rgb8(208, 0, 0)),
                //
                PaintBrush::Color(Color::rgb8(216, 66, 0)),
                PaintBrush::Color(Color::rgb8(221, 101, 0)),
                PaintBrush::Color(Color::rgb8(224, 131, 0)),
                PaintBrush::Color(Color::rgb8(225, 159, 0)),
                //
                // Brightest.
                PaintBrush::Color(Color::rgb8(255, 186, 8)),
            ],
            month_labels: MonthLabels::new(month_labels_font.clone(), number_of_days_in_month),
            month_labels_spacing,
            mood_cell_size: Size::ZERO,
            mood_cells_area_rectangle: Rect::default(),
            mood_value_labels: MoodValueLabels::new(
                mood_value_labels_font,
                NUMBER_OF_MOOD_VALUES_PER_DAY as u8,
            ),
            mood_values_per_day_of_month_index: mood_value_per_day_of_month_index,
            number_of_days_in_month,
            mood_cells_grid_stroke: Stroke {
                stroke_brush: PaintBrush::Color(grid_color),
                stroke_style: Default::default(),
                stroke_width: 1.0,
            },
        }
    }

    ///
    fn cell_x0(&self, column_index: usize) -> f64 {
        self.mood_cells_area_rectangle.x0 + column_index as f64 * self.mood_cell_size.width
    }

    ///
    fn cell_y0(&self, mood_value: MoodValue) -> f64 {
        self.mood_cells_area_rectangle.y1 - (1.0 + mood_value as f64) * self.mood_cell_size.height
    }

    ///
    fn update_layout(&mut self) {
        // Update the mood widget's size.
        self.core.rectangle = self
            .core
            .rectangle
            .with_size(Size::new(self.core.size_constraints.maximum().width, 200.0));

        // Update the mood cells area's rectangle.
        self.mood_cells_area_rectangle = Rect::new(
            self.core.rectangle.x0,
            self.core.rectangle.y0 + self.month_labels_spacing,
            self.core.rectangle.x1,
            self.core.rectangle.y1,
        );

        // Update the mood cell's size.
        self.mood_cell_size = Size::new(
            self.mood_cells_area_rectangle.width() / self.number_of_days_in_month as f64,
            self.mood_cells_area_rectangle.height() / NUMBER_OF_MOOD_VALUES_PER_DAY as f64,
        );
    }
}

impl Widget for MoodWidget {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Update the layout.
        self.update_layout();

        self.core.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: &WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::SetValue(value) => {
                // The given value is a `MoodValuesUpdate`.
                if let Some(mood_values_update) = value.downcast_ref::<MoodValuesUpdate>() {
                    // The day of month index is within range.
                    return if mood_values_update.day_of_month_index < 31 {
                        // Set the given mood values to the given day of month.
                        *self
                            .mood_values_per_day_of_month_index
                            .get_mut(mood_values_update.day_of_month_index as usize)
                            .unwrap() = mood_values_update.mood_values.clone();
                        Ok(())
                    }
                    // The day of month index is out of range.
                    else {
                        Err(WidgetError::CommandNotHandled(
                            self.core.widget_id,
                            format!(
                                "Day of month index {} is out of range",
                                mood_values_update.day_of_month_index
                            ),
                        ))
                    };
                }
            }
            _ => {}
        }

        self.core.handle_command(widget_command)
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        match event {
            Event::MouseDown(mouse_event) => {
                // The mouse is not down within the cells area.
                if !self.mood_cells_area_rectangle.contains(mouse_event.pos) {
                    return;
                }

                // Determine the clicked day of month index.
                let clicked_day_of_month_index: u8 =
                    ((mouse_event.pos.x - self.mood_cells_area_rectangle.x0)
                        / self.mood_cell_size.width) as u8;

                // Determine the clicked mood value.
                let clicked_mood_value: MoodValue = (NUMBER_OF_MOOD_VALUES_PER_DAY
                    - ((mouse_event.pos.y - self.mood_cells_area_rectangle.y0)
                        / self.mood_cell_size.height) as MoodValue)
                    as MoodValue
                    - 1;

                // Get the mood values for the clicked day.
                let mood_values = self
                    .mood_values_per_day_of_month_index
                    .get_mut(clicked_day_of_month_index as usize)
                    .unwrap();

                // The clicked mood value was set already.
                if mood_values.contains(&clicked_mood_value) {
                    // Unset it.
                    mood_values.remove(&clicked_mood_value);
                }
                // The clicked mood value was not set yet.
                else {
                    // Set it.
                    mood_values.insert(clicked_mood_value);
                }

                // Inform the world about the update.
                widget_events.push(WidgetEvent::ValueChanged(
                    self.core.widget_id,
                    Box::new(MoodValuesUpdate {
                        day_of_month_index: clicked_day_of_month_index,
                        mood_values: mood_values.clone(),
                    }),
                ));
            }
            _ => {}
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // Write the month labels.
        self.month_labels.paint(
            piet,
            region,
            self.cell_x0(0),
            self.core.rectangle.y0,
            self.mood_cell_size,
        )?;

        // Fill the mood cells.
        for (column_index, mood_values) in
            self.mood_values_per_day_of_month_index.iter().enumerate()
        {
            for mood_value in mood_values {
                let x = self.cell_x0(column_index);
                let y = self.cell_y0(*mood_value);

                piet.fill(
                    Rect::new(
                        x,
                        y,
                        x + self.mood_cell_size.width,
                        y + self.mood_cell_size.height,
                    ),
                    self.fills_per_mood_value_index
                        .get(*mood_value as usize)
                        .unwrap(),
                );
            }
        }

        // Write the mood numbers for today.
        self.mood_value_labels.paint(
            piet,
            region,
            self.cell_x0(self.day_of_month_index_today),
            self.cell_y0(0),
            self.mood_cell_size,
        )?;

        // Stroke the cell lines.
        {
            // Stroke the vertical lines.
            for column_index in 1..self.number_of_days_in_month {
                let x = self.mood_cells_area_rectangle.x0
                    + column_index as f64 * self.mood_cell_size.width;

                piet.stroke(
                    Line::new(
                        (x, self.mood_cells_area_rectangle.y0),
                        (x, self.mood_cells_area_rectangle.y1),
                    ),
                    &self.mood_cells_grid_stroke.stroke_brush,
                    self.mood_cells_grid_stroke.stroke_width,
                );
            }

            // Stroke the horizontal lines.
            for row_index in 1..NUMBER_OF_MOOD_VALUES_PER_DAY {
                let y = self.mood_cells_area_rectangle.y0
                    + row_index as f64 * self.mood_cell_size.height;

                piet.stroke(
                    Line::new(
                        (self.mood_cells_area_rectangle.x0, y),
                        (self.mood_cells_area_rectangle.x1, y),
                    ),
                    &self.mood_cells_grid_stroke.stroke_brush,
                    self.mood_cells_grid_stroke.stroke_width,
                );
            }

            // Draw a rectangle.
            piet.stroke(
                self.mood_cells_area_rectangle,
                &self.mood_cells_grid_stroke.stroke_brush,
                self.mood_cells_grid_stroke.stroke_width,
            );
        }

        // Render debug hints.
        if self.core.debug_rendering {
            piet.stroke(
                self.core.rectangle,
                &self.core.debug_rendering_stroke.stroke_brush,
                self.core.debug_rendering_stroke.stroke_width,
            );
        }

        Ok(())
    }

    fn rectangle(&self) -> &Rect {
        &self.core.rectangle
    }

    fn set_origin(&mut self, origin: Point) {
        self.core.rectangle = self.core.rectangle.with_origin(origin);

        // Update the layout.
        self.update_layout();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}
