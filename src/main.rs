mod error;
mod view;
mod widget;

use crate::error::ApplicationError;
use crate::widget::{MoodValuesUpdate, MoodWidget};
use guiver::widget::WidgetError;
use guiver::{
    run, Application, Clipboard, Color, Command, Event, PaintBrush, Piet, Region, Size, Stroke,
    StrokeStyle, WidgetEvent, WidgetId, WidgetManager,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// The application data.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct ApplicationData {
    mood_per_day: HashMap<u8, HashSet<u8>>,
}

impl ApplicationData {
    fn new() -> Self {
        ApplicationData {
            mood_per_day: HashMap::new(),
        }
    }
}

// =================================================================================================

/// The application state.
struct ApplicationState {
    application_data: ApplicationData,
    close_button: WidgetId,
    path: PathBuf,
    there_is_unsaved_data: bool,
    widget_manager: WidgetManager,
    widget_mood: WidgetId,
}

impl ApplicationState {
    pub(crate) fn new() -> Result<Self, WidgetError> {
        // TODO: Use a good path.
        let path = PathBuf::from("freundchen.dat");

        let mut widget_manager = WidgetManager::new();

        let debug_rendering_stroke = Stroke {
            stroke_brush: PaintBrush::Color(Color::rgb8(255, 0, 0)),
            stroke_style: StrokeStyle::default(),
            stroke_width: 1.0,
        };

        // Create the widget.
        let padding = widget_manager.new_padding();
        let column = widget_manager.new_column();
        let text = widget_manager.new_text("Hi, how are you today?");
        let widget_mood = widget_manager.next_widget_id();
        let close_button = widget_manager.new_text_button("Close");

        // Add the mood widget.
        widget_manager.add_widget(Box::new(MoodWidget::new(
            widget_mood,
            debug_rendering_stroke.clone(),
        )));

        // Compose the widget.
        widget_manager.send_commands(vec![
            Command::SetMainWidget(padding),
            Command::AddChild(padding, None, column),
            Command::AddChild(column, None, text),
            Command::AddChild(column, None, widget_mood),
            Command::AddChild(column, None, close_button),
        ])?;

        Ok(ApplicationState {
            application_data: ApplicationData::new(),
            close_button,
            path,
            there_is_unsaved_data: false,
            widget_manager,
            widget_mood,
        })
    }

    ///
    fn load_application_data(&mut self) -> Result<(), ApplicationError> {
        // The application data file does not exist.
        if !self.path.exists() {
            return Ok(());
        }

        // Try to read the data file.
        let file_content = fs::read_to_string(&self.path)?;

        // Try to deserialize the application data.
        let deserialized_application_data: ApplicationData = serde_json::from_str(&file_content)?;

        self.application_data = deserialized_application_data.clone();
        self.there_is_unsaved_data = false;

        // Send the new data to the mood widget.
        {
            let mut commands = vec![];

            // TODO: Erase current data.

            for (day_of_month_index, mood_values) in
                deserialized_application_data.mood_per_day.into_iter()
            {
                commands.push(Command::SetValue(
                    self.widget_mood,
                    Box::new(MoodValuesUpdate {
                        day_of_month_index,
                        mood_values,
                    }),
                ));
            }

            self.widget_manager.send_commands(commands)?;
        }

        Ok(())
    }

    /// Saves and quits the application.
    fn save_and_quit(&mut self) {
        // TODO: error handling.
        let _ = self.save_application_data();

        // End the process.
        std::process::exit(0);
    }

    ///
    fn save_application_data(&mut self) -> Result<(), ApplicationError> {
        // There is no unsaved data.
        if !self.there_is_unsaved_data {
            return Ok(());
        }

        // Serialize the application data.
        let serialized_application_data: String = format!("{}", json!(self.application_data));

        // Try to create the serialized application file.
        let mut application_data_file = File::create(&self.path)?;

        // Try to write the serialized application data.
        write!(application_data_file, "{}", serialized_application_data)?;

        self.there_is_unsaved_data = false;

        println!("Saved to \"{}\"", self.path.display());

        Ok(())
    }
}

impl Application for ApplicationState {
    fn handle_event(&mut self, system_event: &Event) {
        // The app was requested to close.
        if *system_event == Event::RequestClose {
            // Save and quit the application.
            self.save_and_quit();
        }

        // TODO: error handling
        let widget_events = self
            .widget_manager
            .handle_event(system_event, None)
            .unwrap();

        // Iterate over the produced widget events.
        for widget_event in widget_events {
            match widget_event {
                WidgetEvent::Clicked(widget_id) => {
                    // The close button was clicked.
                    if widget_id == self.close_button {
                        self.save_and_quit();
                    }
                }
                WidgetEvent::ValueChanged(widget_id, value) => {
                    // A value of the mood widget has changed.
                    if widget_id == self.widget_mood {
                        // The given value is a `MoodValuesUpdate`.
                        if let Some(mood_values_update) = value.downcast_ref::<MoodValuesUpdate>() {
                            // No mood values are given.
                            if mood_values_update.mood_values.is_empty() {
                                // Remove the entry.
                                self.application_data
                                    .mood_per_day
                                    .remove(&mood_values_update.day_of_month_index);
                            }
                            // Mood values are given.
                            else {
                                // Update the mood values in the application data.
                                self.application_data.mood_per_day.insert(
                                    mood_values_update.day_of_month_index,
                                    mood_values_update.mood_values.clone(),
                                );
                            }

                            self.there_is_unsaved_data = true;
                        } else {
                            unimplemented!()
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: Size) {
        self.widget_manager.resize(size);
    }

    fn set_clipboard(&mut self, _clipboard: Clipboard) {}
}

pub fn main() -> Result<(), ApplicationError> {
    // Create the application state.
    let mut application_state = ApplicationState::new()?;

    // Try to load the application data.
    application_state.load_application_data()?;

    // TODO: Call `ApplicationState::save_and_quit()`
    ctrlc::set_handler(move || {
        println!("TODO: call `ApplicationState::save_and_quit()` from the signal handler somehow")
    })
    .expect("Error setting Ctrl+C handler");

    run(
        Box::new(application_state),
        "freundchen",
        (800.0, 600.0).into(),
    );

    Ok(())
}
