use crate::{
    app_state::AppState,
    controller::LayoutSpec,
    states::{InspectOption, LedgerMode, WidgetSlot},
};
use either::Either::{Left, Right};
use ratatui::layout::{Constraint, Direction};

pub fn build_layout_spec(app_state: &AppState) -> LayoutSpec {
    match app_state.inspect_option.current() {
        InspectOption::Ledger => build_ledger_ls(app_state),
        InspectOption::Chain => build_chain_ls(app_state),
        InspectOption::Otel => build_otel_ls(app_state),
    }
}

fn build_ledger_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Fill(1), Right(build_ledger_rest_ls(app_state))),
            (Constraint::Length(1), Left(WidgetSlot::BottomLine)),
        ],
    }
}

fn build_ledger_rest_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (
                Constraint::Length(3),
                Right(build_ledger_header_ls(app_state)),
            ),
            (Constraint::Fill(1), Right(build_ledger_body_ls(app_state))),
        ],
    }
}

fn build_ledger_header_ls(s: &AppState) -> LayoutSpec {
    let constraints = match s.ledger_mode.current() {
        LedgerMode::Browse => vec![
            (Constraint::Fill(1), Left(WidgetSlot::InspectOption)),
            (Constraint::Fill(1), Left(WidgetSlot::LedgerMode)),
        ],
        LedgerMode::Search => vec![
            (Constraint::Length(30), Left(WidgetSlot::InspectOption)),
            (Constraint::Length(20), Left(WidgetSlot::LedgerMode)),
            (Constraint::Fill(1), Left(WidgetSlot::SearchBar)),
        ],
    };
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints,
    }
}

fn build_ledger_body_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (
                Constraint::Percentage(20),
                Right(build_ledger_left_col_ls(app_state)),
            ),
            (Constraint::Percentage(80), Left(WidgetSlot::Details)),
        ],
    }
}

fn build_ledger_left_col_ls(_s: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Fill(1), Left(WidgetSlot::Options)),
            (Constraint::Fill(3), Left(WidgetSlot::List)),
        ],
    }
}

fn build_chain_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Fill(1), Right(build_chain_rest_ls(app_state))),
            (Constraint::Length(1), Left(WidgetSlot::BottomLine)),
        ],
    }
}

fn build_chain_rest_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (
                Constraint::Length(3),
                Right(build_chain_header_ls(app_state)),
            ),
            (Constraint::Fill(1), Right(build_chain_body_ls(app_state))),
        ],
    }
}

fn build_chain_header_ls(_s: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (Constraint::Length(30), Left(WidgetSlot::InspectOption)),
            (Constraint::Fill(1), Left(WidgetSlot::SearchBar)),
        ],
    }
}

fn build_chain_body_ls(_s: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (Constraint::Fill(1), Left(WidgetSlot::LedgerHeaderDetails)),
            (Constraint::Fill(1), Left(WidgetSlot::LedgerBlockDetails)),
            (Constraint::Fill(1), Left(WidgetSlot::LedgerNoncesDetails)),
        ],
    }
}

fn build_otel_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Fill(1), Right(build_otel_rest_ls(app_state))),
            (Constraint::Length(1), Left(WidgetSlot::BottomLine)),
        ],
    }
}

fn build_otel_rest_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (
                Constraint::Length(3),
                Right(build_otel_header_ls(app_state)),
            ),
            (Constraint::Fill(1), Right(build_otel_body_ls(app_state))),
        ],
    }
}

fn build_otel_header_ls(_s: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![(Constraint::Fill(1), Left(WidgetSlot::InspectOption))],
    }
}

fn build_otel_body_ls(_s: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![(Constraint::Fill(1), Left(WidgetSlot::Details))],
    }
}
