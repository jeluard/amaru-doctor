use crate::{
    app_state::AppState,
    cursor::Cursor,
    shared::Shared,
    states::{WidgetId, WidgetSlot},
    ui::{
        to_list_item::ToListItem,
        to_rich::{RichText, ToRichText},
    },
    window::WindowState,
};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, ToLine},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
};

pub type SlotLayout = Vec<(WidgetSlot, Rect)>;

pub fn render_app(frame: &mut Frame, app_state: Shared<AppState>) -> Result<()> {
    for (slot, area) in compute_slot_layout(frame.area()) {
        if let Some(widget_id) = app_state.clone().borrow().get_selected_widget(slot) {
            draw_widget_by_id(frame, area, app_state.clone(), widget_id)?
        }
    }
    Ok(())
}

fn compute_slot_layout(area: Rect) -> SlotLayout {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(area);
    let (left, right) = (columns[0], columns[1]);

    let left_regions = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Fill(1),
        ])
        .split(left);
    let (nav, options, list) = (left_regions[0], left_regions[1], left_regions[2]);

    let right_regions = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(right);
    let (_search_query, details) = (right_regions[0], right_regions[1]);

    vec![
        (WidgetSlot::Nav, nav),
        (WidgetSlot::NavType, options),
        (WidgetSlot::List, list),
        (WidgetSlot::Details, details),
    ]
}

fn draw_widget_by_id(
    frame: &mut Frame,
    area: Rect,
    app_state: Shared<AppState>,
    widget_id: WidgetId,
) -> Result<()> {
    match widget_id {
        WidgetId::Empty => draw_empty(frame, area, app_state),
        WidgetId::CursorTabs => {
            let tabs = app_state.borrow().tabs.clone();
            draw_cursor(frame, area, app_state, tabs, widget_id)
        }
        WidgetId::ListBrowseOptions => {
            let list = app_state.borrow().browse_options.clone();
            draw_list(frame, area, app_state, list, widget_id)
        }
        WidgetId::ListSearchOptions => {
            let list = app_state.borrow().search_options.clone();
            draw_list(frame, area, app_state, list, widget_id)
        }
        WidgetId::ListAccounts => {
            let list = app_state.borrow().accounts.clone();
            draw_list(frame, area, app_state, list, widget_id)
        }
        WidgetId::ListBlockIssuers => {
            let list = app_state.borrow().block_issuers.clone();
            draw_list(frame, area, app_state, list, widget_id)
        }
        WidgetId::ListDReps => {
            let list = app_state.borrow().dreps.clone();
            draw_list(frame, area, app_state, list, widget_id)
        }
        WidgetId::ListPools => {
            let list = app_state.borrow().pools.clone();
            draw_list(frame, area, app_state, list, widget_id)
        }
        WidgetId::ListProposals => {
            let list = app_state.borrow().proposals.clone();
            draw_list(frame, area, app_state, list, widget_id)
        }
        WidgetId::ListUtxos => {
            let list = app_state.borrow().utxos.clone();
            draw_list(frame, area, app_state, list, widget_id)
        }
        // TODO: Impl detail renderer
        WidgetId::DetailAccount => {
            let list = app_state.borrow().accounts.clone();
            draw_detail(frame, area, app_state, list, widget_id)
        }
        WidgetId::DetailBlockIssuer => {
            let list = app_state.borrow().block_issuers.clone();
            draw_detail(frame, area, app_state, list, widget_id)
        }
        WidgetId::DetailDRep => {
            let list = app_state.borrow().dreps.clone();
            draw_detail(frame, area, app_state, list, widget_id)
        }
        WidgetId::DetailPool => {
            let list = app_state.borrow().pools.clone();
            draw_detail(frame, area, app_state, list, widget_id)
        }
        WidgetId::DetailProposal => {
            let list = app_state.borrow().proposals.clone();
            draw_detail(frame, area, app_state, list, widget_id)
        }
        WidgetId::DetailUtxo => {
            let list = app_state.borrow().utxos.clone();
            draw_detail(frame, area, app_state, list, widget_id)
        }
    }
}

fn draw_empty(frame: &mut Frame, area: Rect, app_state: Shared<AppState>) -> Result<()> {
    let mut block = Block::default()
        .title("Empty")
        .title_style(Style::default().fg(Color::White))
        .borders(Borders::ALL);

    if app_state.borrow().is_widget_focused(WidgetId::Empty) {
        block = block.border_style(Style::default().fg(Color::Blue));
    }

    frame.render_widget(block, area);
    Ok(())
}

fn draw_cursor<T: ToLine>(
    frame: &mut Frame<'_>,
    area: Rect,
    app_state: Shared<AppState>,
    tabs: Shared<Cursor<T>>,
    widget_id: WidgetId,
) -> Result<()> {
    match widget_id {
        WidgetId::CursorTabs => {
            let mut block = Block::default()
                .borders(Borders::ALL)
                .title(serde_plain::to_string(&widget_id)?);

            if app_state.borrow().is_widget_focused(widget_id) {
                block = block
                    .border_style(Style::default().fg(Color::Blue))
                    .title_style(Style::default().fg(Color::White));
            }
            let tab_brw = tabs.borrow();
            let tab_lis: Vec<Line> = tab_brw.iter().map(ToLine::to_line).collect();
            let tabs = Tabs::new(tab_lis)
                .select(tabs.borrow().index())
                .block(block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            frame.render_widget(tabs, area);
        }
        _ => panic!("Widget id is not a cursor"),
    }
    Ok(())
}

fn draw_list<T: ToListItem>(
    frame: &mut Frame,
    area: Rect,
    app_state: Shared<AppState>,
    list: Shared<WindowState<T>>,
    widget_id: WidgetId,
) -> Result<()> {
    // TODO: Capture somewhere else so that this doesn't need to be mut
    list.borrow_mut().set_window_size(area.rows().count());

    let binding = list.borrow();
    let (view, selected) = binding.window_view();
    let items: Vec<ListItem> = view.iter().map(|i| i.to_list_item()).collect();

    let mut block = Block::default()
        .title(serde_plain::to_string(&widget_id)?)
        .borders(Borders::ALL);
    if app_state.borrow().is_widget_focused(widget_id) {
        block = block
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::White));
    }

    let list = List::new(items)
        .highlight_symbol(">> ")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .block(block);

    let mut state = ListState::default();
    state.select(Some(selected));
    frame.render_stateful_widget(list, area, &mut state);

    Ok(())
}

fn draw_detail<T: ToRichText>(
    frame: &mut Frame,
    area: Rect,
    app_state: Shared<AppState>,
    list: Shared<WindowState<T>>,
    widget_id: WidgetId,
) -> Result<()> {
    let mut block = Block::default()
        .title(serde_plain::to_string(&widget_id)?)
        .borders(Borders::ALL);

    if app_state.borrow().is_widget_focused(widget_id) {
        block = block
            .title_style(Style::default().fg(Color::White))
            .border_style(Style::default().fg(Color::Blue));
    }

    let lines = list
        .borrow()
        .selected()
        .map_or(RichText::Single(Span::raw("None selected")), |t| {
            t.to_rich_text()
        })
        .unwrap_lines();
    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    // TODO: Add offset state to AppState
    // .scroll((self.scroll_offset, 0));
    frame.render_widget(paragraph, area);
    Ok(())
}
