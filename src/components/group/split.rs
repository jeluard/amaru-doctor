use crate::{
    action::Action,
    components::{Component, group::ComponentGroup},
    config::Config,
    shared::Shared,
    tui::Event,
};
use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use delegate::delegate;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Size},
};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone, Copy, Default)]
pub enum Axis {
    #[default]
    Horizontal,
    Vertical,
}

pub struct SplitComponent<'a> {
    axis: Axis,
    percents: Vec<u16>,
    group: ComponentGroup<'a>,
}

impl<'a> SplitComponent<'a> {
    pub fn new_n(
        axis: Axis,
        ratios: Vec<u16>,
        components: Vec<Shared<dyn Component + 'a>>,
    ) -> Self {
        assert_eq!(ratios.len(), components.len());
        assert_eq!(ratios.iter().sum::<u16>(), 100);
        Self {
            axis,
            percents: ratios,
            group: ComponentGroup::new(components),
        }
    }

    pub fn new_2(
        axis: Axis,
        ratio_a: u16,
        comp_a: Shared<dyn Component + 'a>,
        ratio_b: u16,
        comp_b: Shared<dyn Component + 'a>,
    ) -> Self {
        Self::new_n(axis, vec![ratio_a, ratio_b], vec![comp_a, comp_b])
    }

    pub fn new_2_evenly(
        axis: Axis,
        comp_a: Shared<dyn Component + 'a>,
        comp_b: Shared<dyn Component + 'a>,
    ) -> Self {
        Self::new_2(axis, 50, comp_a, 50, comp_b)
    }
}

impl<'a> Component for SplitComponent<'a> {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let constraints: Vec<Constraint> = self
            .percents
            .iter()
            .map(|r| Constraint::Percentage(*r))
            .collect();

        let chunks = match self.axis {
            Axis::Horizontal => Layout::vertical(constraints).split(area),
            Axis::Vertical => Layout::horizontal(constraints).split(area),
        };

        for (component, region) in self.group.iter_mut().zip(chunks.iter()) {
            component.borrow_mut().draw(frame, *region)?;
        }

        Ok(())
    }

    delegate! {
        to self.group {
            fn update(&mut self, action: Action) -> Result<Vec<Action>>;
            fn handle_events(&mut self, event: Option<Event>) -> Result<Vec<Action>>;
            fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>>;
            fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<Vec<Action>>;
            fn init(&mut self, area: Size) -> Result<()>;
            fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()>;
            fn register_config_handler(&mut self, config: Config) -> Result<()>;
        }
    }
}
