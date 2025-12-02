use crate::{
    components::Component,
    model::search::SearchCache,
    states::{Action, ComponentId},
    view::item_details::draw_details,
};
use amaru_consensus::{BlockHeader, Nonces, ReadOnlyChainStore};
use amaru_kernel::{Hash, RawBlock};
use amaru_stores::rocksdb::consensus::ReadOnlyChainDB;
use crossterm::event::{Event, MouseButton, MouseEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};
use std::{any::Any, str::FromStr, sync::Arc};
use tracing::warn;

pub type ChainResult = (BlockHeader, RawBlock, Nonces);

pub struct ChainSearchComponent {
    id: ComponentId,
    db: Arc<ReadOnlyChainDB>,
    state: SearchCache<Hash<32>, ChainResult>,
    focused_column: Option<usize>,
}

impl ChainSearchComponent {
    pub fn new(id: ComponentId, db: Arc<ReadOnlyChainDB>) -> Self {
        Self {
            id,
            db,
            state: SearchCache::default(),
            focused_column: None,
        }
    }

    fn get_layout_chunks(&self, area: Rect) -> std::rc::Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(area)
    }

    pub fn handle_search(&mut self, query_str: &str) {
        let Ok(hash) = Hash::<32>::from_str(query_str) else {
            warn!("Invalid hash format: {}", query_str);
            return;
        };

        // Check Cache
        if self.state.results.contains_key(&hash) {
            self.state.parsed = Some(hash);
            return;
        }

        let header_opt = self.db.load_header(&hash);
        let block_res = ReadOnlyChainStore::<BlockHeader>::load_block(&*self.db, &hash);
        let nonces_opt = ReadOnlyChainStore::<BlockHeader>::get_nonces(&*self.db, &hash);

        if let (Some(header), Ok(block), Some(nonces)) = (header_opt, block_res, nonces_opt) {
            self.state.cache_result(hash, (header, block, nonces));
        }
    }

    pub fn render_focused(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        let chunks = self.get_layout_chunks(area);

        let result = self.state.get_current_res();
        let header = result.map(|r| &r.0);
        let block = result.map(|r| &r.1);
        let nonces = result.map(|r| &r.2);

        // Highlight specific columns if globally focused (and specific column selected)
        let f0 = is_focused && self.focused_column == Some(0);
        let f1 = is_focused && self.focused_column == Some(1);
        let f2 = is_focused && self.focused_column == Some(2);

        draw_details(f, chunks[0], "Header Details".to_string(), header, f0);
        draw_details(f, chunks[1], "Block Details".to_string(), block, f1);
        draw_details(f, chunks[2], "Nonces Details".to_string(), nonces, f2);
    }
}

impl Component for ChainSearchComponent {
    fn id(&self) -> ComponentId {
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        if let Event::Mouse(mouse) = event
            && (mouse.kind == MouseEventKind::Moved
                || mouse.kind == MouseEventKind::Down(MouseButton::Left))
        {
            let chunks = self.get_layout_chunks(area);

            self.focused_column = None;
            for (i, rect) in chunks.iter().enumerate() {
                if mouse.column >= rect.x
                    && mouse.column < rect.x + rect.width
                    && mouse.row >= rect.y
                    && mouse.row < rect.y + rect.height
                {
                    self.focused_column = Some(i);
                }
            }
        }
        Vec::new()
    }
}
