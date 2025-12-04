use crate::{
    components::{Component, ComponentLayout},
    controller::{LayoutSpec, MoveFocus, find_next_focus, walk_layout},
    metrics::{
        charts::{ChartDatasetConfig, render_chart},
        model::{AmaruMetric, MetricUpdate, NodeMetrics},
        service,
    },
    states::{Action, ComponentId},
};
use either::Either::Left;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};
use std::{any::Any, collections::HashMap, sync::RwLock};
use tokio::sync::mpsc::{self, Receiver};

pub struct MetricsPageComponent {
    id: ComponentId,
    pub metrics: NodeMetrics,
    update_rx: Receiver<MetricUpdate>,
    last_layout: RwLock<ComponentLayout>,
    active_focus: RwLock<ComponentId>,
}

impl MetricsPageComponent {
    pub fn new(update_rx: Receiver<MetricUpdate>) -> Self {
        Self {
            id: ComponentId::MetricsPage,
            metrics: NodeMetrics::default(),
            update_rx,
            last_layout: RwLock::new(HashMap::new()),
            active_focus: RwLock::new(ComponentId::MetricsPage),
        }
    }

    pub fn new_with_service() -> Self {
        let (tx, rx) = mpsc::channel(100);
        service::start(tx);
        Self::new(rx)
    }

    fn process_update(&mut self, update: MetricUpdate) {
        self.metrics.handle_update(update);
    }

    pub fn handle_navigation(&mut self, direction: MoveFocus) -> Vec<Action> {
        let layout = self.last_layout.read().unwrap();
        let active_focus = *self.active_focus.read().unwrap();

        if let Some(next) = find_next_focus(&layout, active_focus, direction) {
            *self.active_focus.write().unwrap() = next;
            return vec![Action::SetFocus(next)];
        }

        Vec::new()
    }

    pub fn calculate_layout(&self, area: Rect) -> ComponentLayout {
        let spec = LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![(Constraint::Fill(1), Left(ComponentId::Metrics))],
        };

        let mut layout = HashMap::new();
        walk_layout(&mut layout, &spec, area);
        layout
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33), // CPU
                Constraint::Percentage(33), // Memory
                Constraint::Percentage(34), // Disk
            ])
            .split(area);

        // --- Chart 1: CPU Usage ---
        render_chart(
            f,
            chunks[0],
            &[ChartDatasetConfig {
                metric: AmaruMetric::ProcessCpuLive,
                data: &self.metrics.process_cpu_live,
                label: "CPU Util",
                color: Color::Cyan,
            }],
            "CPU",
        );

        // --- Chart 2: Memory Usage ---
        render_chart(
            f,
            chunks[1],
            &[ChartDatasetConfig {
                metric: AmaruMetric::ProcessMemoryLiveResident,
                data: &self.metrics.process_memory_live_resident,
                label: "Memory",
                color: Color::Green,
            }],
            " Memory",
        );

        // --- Chart 3: Disk I/O (Dual Series) ---
        render_chart(
            f,
            chunks[2],
            &[
                ChartDatasetConfig {
                    metric: AmaruMetric::ProcessDiskLiveRead,
                    data: &self.metrics.process_disk_live_read,
                    label: "Read",
                    color: Color::Yellow,
                },
                ChartDatasetConfig {
                    metric: AmaruMetric::ProcessDiskLiveWrite,
                    data: &self.metrics.process_disk_live_write,
                    label: "Write",
                    color: Color::Cyan,
                },
            ],
            " Disk I/O",
        );
    }
}

impl Component for MetricsPageComponent {
    fn id(&self) -> ComponentId {
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn tick(&mut self) -> Vec<Action> {
        const MAX_UPDATES_PER_TICK: usize = 100;
        let mut count = 0;
        while count < MAX_UPDATES_PER_TICK {
            match self.update_rx.try_recv() {
                Ok(update) => {
                    self.process_update(update);
                    count += 1;
                }
                Err(_) => break,
            }
        }
        Vec::new()
    }
}
