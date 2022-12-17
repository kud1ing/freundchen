mod dashboard;

use crate::ApplicationError;
use guiver::{WidgetEvent, WidgetId, WidgetManager};
use std::cell::RefCell;
use std::rc::Rc;

/// A command to the application.
#[derive(Clone)]
pub(crate) enum ApplicationCommand {
    ShowDashboard,
}

///
pub(crate) trait View {
    ///
    fn activate(
        &mut self,
        application_command: ApplicationCommand,
        widget_manager: &mut WidgetManager<()>,
    ) -> Result<(), ApplicationError>;

    ///
    fn handle_event(
        &mut self,
        widget_manager: &mut WidgetManager<()>,
        widget_events: &[WidgetEvent],
    ) -> Option<ApplicationCommand>;

    /// The ID of the view's main widget.
    fn main_widget_id(&self) -> &WidgetId;
}

///
type ViewBox = Rc<RefCell<Box<dyn View>>>;
