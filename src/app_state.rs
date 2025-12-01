use crate::{
    ScreenMode,
    model::{
        button::InputEvent, chain_view::ChainViewState, layout::LayoutModel,
        ledger_view::LedgerModelViewState, otel_view::OtelViewState,
    },
    otel::graph::TraceGraph,
    states::{ComponentId, InspectOption, LedgerMode},
    update::mouse::MouseState,
};
use anyhow::Result;
use arc_swap::ArcSwap;
use ratatui::layout::Rect;
use std::sync::{Arc, mpsc};

/// Holds ALL the app's state. Does not self-mutate.
pub struct AppState {
    pub screen_mode: ScreenMode,

    pub frame_area: Rect,
    pub layout_model: LayoutModel,

    pub ledger_mvs: LedgerModelViewState,
    pub chain_view: ChainViewState,

    pub otel_view: OtelViewState,

    pub button_events: mpsc::Receiver<InputEvent>,

    pub mouse_state: MouseState,

    pub focused_component: ComponentId,
}

impl AppState {
    pub fn new(
        trace_graph: Arc<ArcSwap<TraceGraph>>,
        button_events: mpsc::Receiver<InputEvent>,
        frame_area: Rect,
        screen_mode: ScreenMode,
    ) -> Result<Self> {
        let layout_model = LayoutModel::new(
            screen_mode,
            InspectOption::default(),
            LedgerMode::default(),
            frame_area,
        );

        let options_height = 0;
        let list_height = 0;

        Ok(Self {
            screen_mode,
            frame_area: Rect::default(),
            layout_model,
            ledger_mvs: LedgerModelViewState::new(options_height, list_height),
            chain_view: ChainViewState::default(),
            otel_view: OtelViewState::new(trace_graph),
            button_events,
            mouse_state: MouseState::default(),
            focused_component: ComponentId::InspectTabs,
        })
    }
}
