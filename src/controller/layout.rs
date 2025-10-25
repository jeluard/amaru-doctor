use crate::{
    ScreenMode,
    controller::LayoutSpec,
    states::{InspectOption, LedgerMode, WidgetSlot},
};
use either::Either::{Left, Right};
use ratatui::layout::{Constraint, Direction};

// TODO: Use a builder in here
pub fn build_layout_spec(
    screen_mode: ScreenMode,
    inspect_tabs: InspectOption,
    ledger_mode: LedgerMode,
) -> LayoutSpec {
    match inspect_tabs {
        InspectOption::Ledger => build_ledger_ls(screen_mode, ledger_mode),
        //InspectOption::Chain => build_chain_ls(),
        InspectOption::Otel => build_otel_ls(),
        InspectOption::Prometheus => build_prom_ls(),
    }
}

fn build_ledger_ls(screen_mode: ScreenMode, ledger_mode: LedgerMode) -> LayoutSpec {
    match screen_mode {
        ScreenMode::Large => LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![
                (
                    Constraint::Fill(1),
                    Right(build_ledger_rest_ls(screen_mode, ledger_mode)),
                ),
                (Constraint::Length(1), Left(WidgetSlot::BottomLine)),
            ],
        },
        // No bottom bar for small screens
        ScreenMode::Small => LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![(
                Constraint::Fill(1),
                Right(build_ledger_rest_ls(screen_mode, ledger_mode)),
            )],
        },
    }
}

fn build_ledger_rest_ls(screen_mode: ScreenMode, ledger_mode: LedgerMode) -> LayoutSpec {
    match screen_mode {
        ScreenMode::Large => LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![
                (
                    Constraint::Length(3),
                    Right(build_ledger_header_ls(ledger_mode)),
                ),
                (Constraint::Fill(1), Right(build_ledger_body_ls())),
            ],
        },
        ScreenMode::Small => LayoutSpec {
            // Small screen only has the body
            direction: Direction::Vertical,
            constraints: vec![(Constraint::Fill(1), Right(build_ledger_body_ls()))],
        },
    }
}

fn build_ledger_header_ls(ledger_mode: LedgerMode) -> LayoutSpec {
    let constraints = match ledger_mode {
        LedgerMode::Browse => vec![
            (Constraint::Fill(1), Left(WidgetSlot::InspectOption)),
            //(Constraint::Fill(1), Left(WidgetSlot::LedgerMode)),
        ],
        LedgerMode::Search => vec![
            (Constraint::Length(30), Left(WidgetSlot::InspectOption)),
            //(Constraint::Length(20), Left(WidgetSlot::LedgerMode)),
            //(Constraint::Fill(1), Left(WidgetSlot::SearchBar)),
        ],
    };
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints,
    }
}

fn build_ledger_body_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (
                Constraint::Percentage(20),
                Right(build_ledger_left_col_ls()),
            ),
            (Constraint::Percentage(80), Left(WidgetSlot::Details)),
        ],
    }
}

fn build_ledger_left_col_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Fill(1), Left(WidgetSlot::LedgerOptions)),
            (Constraint::Fill(3), Left(WidgetSlot::List)),
        ],
    }
}

#[allow(dead_code)]
fn build_chain_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Fill(1), Right(build_chain_rest_ls())),
            (Constraint::Length(1), Left(WidgetSlot::BottomLine)),
        ],
    }
}

fn build_chain_rest_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Length(3), Right(build_chain_header_ls())),
            (Constraint::Fill(1), Right(build_chain_body_ls())),
        ],
    }
}

fn build_chain_header_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (Constraint::Fill(1), Left(WidgetSlot::InspectOption)),
            //(Constraint::Fill(1), Left(WidgetSlot::SearchBar)),
        ],
    }
}

fn build_chain_body_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (Constraint::Fill(1), Left(WidgetSlot::LedgerHeaderDetails)),
            (Constraint::Fill(1), Left(WidgetSlot::LedgerBlockDetails)),
            (Constraint::Fill(1), Left(WidgetSlot::LedgerNoncesDetails)),
        ],
    }
}

fn build_otel_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Fill(1), Right(build_otel_rest_ls())),
            (Constraint::Length(1), Left(WidgetSlot::BottomLine)),
        ],
    }
}

fn build_otel_rest_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Length(3), Right(build_otel_header_ls())),
            (Constraint::Fill(1), Right(build_otel_body_ls())),
        ],
    }
}

fn build_otel_header_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![(Constraint::Fill(1), Left(WidgetSlot::InspectOption))],
    }
}

fn build_otel_body_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (Constraint::Percentage(10), Left(WidgetSlot::List)),
            (Constraint::Percentage(90), Right(build_otel_details_ls())),
        ],
    }
}

fn build_otel_details_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (Constraint::Percentage(70), Left(WidgetSlot::Details)),
            (Constraint::Percentage(30), Left(WidgetSlot::SubDetails)),
        ],
    }
}

fn build_prom_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Fill(1), Right(build_prom_rest_ls())),
            (Constraint::Length(1), Left(WidgetSlot::BottomLine)),
        ],
    }
}

fn build_prom_rest_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Length(3), Right(build_prom_header_ls())),
            (Constraint::Fill(1), Right(build_prom_body_ls())),
        ],
    }
}

fn build_prom_header_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![(Constraint::Fill(1), Left(WidgetSlot::InspectOption))],
    }
}

fn build_prom_body_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![(Constraint::Fill(1), Left(WidgetSlot::Details))],
    }
}
