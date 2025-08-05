/// Messages sent from handlers to App for global processing
#[derive(Debug)]
pub enum AppMessage {
    Quit,
    OpenFile,
    SwitchToExploreHandler,
    SwitchToSearchHandler,
    SwitchToFuzzyFindHandler,
    Error(String),
}
