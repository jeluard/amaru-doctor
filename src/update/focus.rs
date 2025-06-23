use crate::{
    app_state::AppState,
    controller::SlotLayout,
    states::{Action, WidgetSlot},
    update::Update,
};
use tracing::trace;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct FocusUpdate;
impl Update for FocusUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Vec<Action> {
        let dir = match action {
            Action::FocusUp => Direction::Up,
            Action::FocusDown => Direction::Down,
            Action::FocusLeft => Direction::Left,
            Action::FocusRight => Direction::Right,
            _ => return vec![],
        };

        trace!("Will shift focus {:?} from {}", dir, app_state.slot_focus);

        if let Some(next) = resolve_directional_focus(app_state.slot_focus, &app_state.layout, &dir)
        {
            app_state.slot_focus = next;
            trace!("Did shift focus {:?} to {}", dir, next);
        }

        vec![]
    }
}

fn resolve_directional_focus(
    current: WidgetSlot,
    layout: &SlotLayout,
    dir: &Direction,
) -> Option<WidgetSlot> {
    let &curr_rect = layout.get(&current)?;

    layout
        .iter()
        .filter(|(slot, _)| **slot != current && WidgetSlot::focusable().contains(slot))
        .filter_map(|(slot, rect)| {
            let valid = match dir {
                Direction::Up => rect.y + rect.height <= curr_rect.y,
                Direction::Down => rect.y >= curr_rect.y + curr_rect.height,
                Direction::Left => rect.x + rect.width <= curr_rect.x,
                Direction::Right => rect.x >= curr_rect.x + curr_rect.width,
            };
            if !valid {
                return None;
            }

            let dx = rect.x.abs_diff(curr_rect.x);
            let dy = rect.y.abs_diff(curr_rect.y);
            Some((slot, dx + dy))
        })
        .min_by_key(|(_, dist)| *dist)
        .map(|(slot, _)| *slot)
}
