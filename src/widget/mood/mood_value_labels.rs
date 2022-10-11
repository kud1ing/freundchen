use guiver::{Error, Font, Piet, PietTextLayout, Rect, Region, RenderContext, Size, TextLayout};
use std::u8;

/// Caches the `PietTextLayout` for the mood value labels.
pub(crate) struct MoodValueLabels {
    text_layouts: Vec<PietTextLayout>,
}

impl MoodValueLabels {
    ///
    pub(crate) fn new(font: Font, number_mood_values: u8) -> Self {
        let mut text_layouts = vec![];

        for mood_value in 0..number_mood_values {
            let text = u8::to_string(&mood_value);
            text_layouts.push(font.text_layout(text));
        }

        MoodValueLabels { text_layouts }
    }

    ///
    pub(crate) fn paint(
        &self,
        piet: &mut Piet,
        _region: &Region,
        x: f64,
        y: f64,
        cell_size: Size,
    ) -> Result<(), Error> {
        for (mood_value_index, text_layout) in self.text_layouts.iter().enumerate() {
            let text_x = x + 0.5 * (cell_size.width - text_layout.size().width).max(0.0);
            let text_y = y
                // Add the current cell's offset.
                - mood_value_index as f64 * cell_size.height
                // Center the text within the cell's height.
                + 0.5 * (cell_size.height - text_layout.size().height);

            // Draw the current mood value.
            piet.save()?;
            piet.clip(Rect::from_origin_size((text_x, text_y), cell_size));
            piet.draw_text(&text_layout, (text_x, text_y));
            piet.restore()?;
        }

        Ok(())
    }
}
