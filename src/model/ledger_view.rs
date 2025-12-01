/// Holds the model state (underlying data) and view state (ui) for the Ledger
/// page
pub struct LedgerModelViewState {
    pub options_window_height: usize,
    pub list_window_height: usize,
}

impl LedgerModelViewState {
    pub fn new(options_window_height: usize, list_window_height: usize) -> Self {
        Self {
            options_window_height,
            list_window_height,
        }
    }
}
