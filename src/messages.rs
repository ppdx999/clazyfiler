/// Messages sent from handlers to App for global processing
#[derive(Debug)]
pub enum AppMessage {
    Quit,
    OpenFile,
    NavigateToDirectory(std::path::PathBuf),
    SwitchToExploreHandler,
    SwitchToSearchHandler,
    SwitchToFuzzyFindHandler,
    Error(String),
}
