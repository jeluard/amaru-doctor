use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::Widget,
};
use std::time::{Duration, Instant};
use tachyonfx::{EffectManager, EffectTimer, Interpolation, fx};

pub struct LogoScreen {
    pub effects: EffectManager<()>,
    triggered: bool,
    logo_area: Option<Rect>,
    duration: Duration,
    start_time: Option<Instant>,
    last_frame_time: Option<Instant>,
    since_last_frame: Duration,
}

const LOGO: &str = indoc::indoc! {"
    ▄▀▀▄  █▄ ▄█ ▄▀▀▄  █▀▀▄ █  █
    █▀▀█  █ ▀ █ █▀▀█  █▀▀▄ ▀▄▄▀
"};

impl LogoScreen {
    pub fn new(duration: Duration) -> Self {
        Self {
            effects: EffectManager::default(),
            triggered: false,
            logo_area: None,
            duration,
            start_time: None,
            last_frame_time: None,
            since_last_frame: Duration::ZERO,
        }
    }

    pub fn is_done(&self) -> bool {
        self.start_time
            .is_some_and(|start_time| start_time.elapsed() >= self.duration)
    }

    fn trigger_explosion(&mut self) {
        if let Some(area) = self.logo_area {
            let effect = fx::explode(
                15.0,
                2.0,
                EffectTimer::new(self.duration.into(), Interpolation::Linear),
            )
            .with_pattern(tachyonfx::pattern::RadialPattern::center())
            .with_filter(tachyonfx::CellFilter::Area(area));
            self.effects.add_effect(effect);
            self.triggered = true;
        }
    }

    pub fn display(&mut self, frame: &mut Frame) {
        let now = Instant::now();
        self.since_last_frame = self
            .last_frame_time
            .map(|last| now.duration_since(last))
            .unwrap_or_default();

        self.last_frame_time = Some(now);

        let area = frame.area();
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Min(3),
                Constraint::Percentage(40),
            ])
            .split(area);
        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Min(20),
                Constraint::Percentage(20),
            ])
            .split(vertical[1]);
        let centered = horizontal[1];
        self.logo_area = Some(centered);
        frame.render_widget(&mut *self, centered);

        if !self.triggered {
            self.start_time = Some(now);
            self.trigger_explosion();
        }
    }
}

impl Widget for &mut LogoScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Text::raw(LOGO).render(area, buf);
        if self.start_time.is_some() {
            self.effects
                .process_effects(self.since_last_frame.into(), buf, area);
        }
    }
}
