use crate::{app_state::AppState, states::Action, update::Update};

/// The Update fn for sync'ing the TraceGraph.
pub struct TraceGraphUpdate;
impl Update for TraceGraphUpdate {
    fn update(&self, a: &Action, s: &mut AppState) -> Vec<Action> {
        if *a == Action::SyncTraceGraph {
            s.otel_view.sync_state();
        }
        Vec::new()
    }
}
