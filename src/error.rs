use guiver::widget::WidgetError;

#[derive(Debug)]
pub enum ApplicationError {
    Io(std::io::Error),
    Deserde(serde_json::Error),
    Widget(WidgetError),
}

impl From<std::io::Error> for ApplicationError {
    fn from(error: std::io::Error) -> Self {
        ApplicationError::Io(error)
    }
}

impl From<serde_json::Error> for ApplicationError {
    fn from(error: serde_json::Error) -> Self {
        ApplicationError::Deserde(error)
    }
}

impl From<WidgetError> for ApplicationError {
    fn from(error: WidgetError) -> Self {
        ApplicationError::Widget(error)
    }
}
