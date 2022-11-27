use guiver::{Error, Font, Piet, PietTextLayout, Rect, Region, RenderContext, Size, TextLayout};

/// Caches the `PietTextLayout` for the month labels.
pub(crate) struct MonthLabels {
    text_layouts: Vec<PietTextLayout>,
}

impl MonthLabels {
    ///
    pub(crate) fn new(font: Font, number_of_days_in_month: u8) -> Self {
        let mut text_layouts = vec![];

        for column_index in 0..number_of_days_in_month {
            let text = u8::to_string(&(column_index + 1));
            text_layouts.push(font.text_layout(text));
        }

        MonthLabels { text_layouts }
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
        for (column_index, text_layout) in self.text_layouts.iter().enumerate() {
            let text_x = x
                // Add the current cell's offset.
                + column_index as f64 * cell_size.width
                // Center the text within the cell's width.
                + 0.5 * (cell_size.width - text_layout.size().width).max(0.0);

            // Draw the current month label.
            piet.save()?;
            piet.clip(Rect::from_origin_size((text_x, y), cell_size));
            piet.draw_text(text_layout, (text_x, y));
            piet.restore()?;
        }

        Ok(())
    }
}
