use crate::{
    app_state::AppState,
    states::Action,
    update::{
        button::GetButtonEventsUpdate, focus::FocusUpdate, layout::LayoutUpdate,
        mouse::MouseEventUpdate, mouse_click::MouseClickUpdate, mouse_focus::MouseFocusUpdate,
        mouse_scroll::MouseScrollUpdate, poll_search::PollUtxoSearchUpdate,
        prom_metrics::PromMetricsUpdate, scroll::ScrollUpdate, search::SearchUpdate,
        select_span::SelectSpanUpdate, tabs::TabsUpdate, tick::TickUpdate,
        trace_graph::TraceGraphUpdate, window::WindowSizeUpdate,
    },
};

pub mod button;
pub mod focus;
pub mod layout;
pub mod mouse;
pub mod mouse_click;
pub mod mouse_focus;
pub mod mouse_scroll;
pub mod poll_search;
pub mod prom_metrics;
pub mod scroll;
pub mod search;
pub mod select_span;
pub mod tabs;
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
    &PollUtxoSearchUpdate,
    &LayoutUpdate,
    &TickUpdate,
    &TraceGraphUpdate,
    &SelectSpanUpdate,
    &PromMetricsUpdate,
    &GetButtonEventsUpdate,
    &MouseEventUpdate,
    &MouseFocusUpdate,
    &MouseClickUpdate,
    &MouseScrollUpdate,
    &TabsUpdate,
];
