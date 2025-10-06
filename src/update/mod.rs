use crate::{
    app_state::AppState,
    states::Action,
    update::{
        focus::FocusUpdate, layout::LayoutUpdate, prom_metrics::PromMetricsUpdate,
        scroll::ScrollUpdate, search::SearchUpdate, select_span::SelectSpanUpdate,
        tick::TickUpdate, trace_graph::TraceGraphUpdate, window::WindowSizeUpdate,
    },
};

pub mod focus;
pub mod layout;
pub mod prom_metrics;
pub mod scroll;
pub mod search;
pub mod select_span;
pub mod tick;
pub mod trace_graph;
pub mod window;

pub type UpdateList = Vec<&'static dyn Update>;
pub trait Update: Sync {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Vec<Action>;
}

pub static UPDATE_DEFS: &[&dyn Update] = &[
    &FocusUpdate,
    &ScrollUpdate,
    &WindowSizeUpdate,
    &SearchUpdate,
    &LayoutUpdate,
    &TickUpdate,
    &TraceGraphUpdate,
    &SelectSpanUpdate,
    &PromMetricsUpdate,
];
