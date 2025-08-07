/// Messages sent from handlers to App for global processing
#[derive(Debug)]
pub enum AppMessage {
    Quit,
    OpenFile,
    SwitchToExploreHandler,
    SwitchToExploreHandlerKeepQuery,  // Keep search results when switching to explore mode
    SwitchToSearchHandler,
    SwitchToFuzzyFindHandler,
    Error(String),
}
