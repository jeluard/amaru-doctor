use crate::{
    ScreenMode,
    controller::{LayoutContext, LayoutSpec},
    states::{ComponentId, InspectOption, LedgerBrowse, LedgerMode, LedgerSearch},
};
use either::Either::{Left, Right};
use ratatui::layout::{Constraint, Direction};

pub fn build_layout_spec(ctx: &LayoutContext) -> LayoutSpec {
    match ctx.inspect_option {
        InspectOption::Ledger => build_ledger_ls(ctx),
        InspectOption::Chain => build_chain_ls(),
        InspectOption::Otel => build_otel_ls(),
        InspectOption::Prometheus => build_prom_ls(),
    }
}

fn build_ledger_ls(ctx: &LayoutContext) -> LayoutSpec {
    match ctx.screen_mode {
        ScreenMode::Large | ScreenMode::Small => LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![(Constraint::Fill(1), Right(build_ledger_rest_ls(ctx)))],
        },
    }
}

fn build_ledger_rest_ls(ctx: &LayoutContext) -> LayoutSpec {
    match ctx.screen_mode {
        ScreenMode::Large => LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![
                (
                    Constraint::Length(3),
                    Right(build_ledger_header_ls(ctx.ledger_mode)),
                ),
                (Constraint::Fill(1), Right(build_ledger_body_ls(ctx))),
            ],
        },
        ScreenMode::Small => LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![(Constraint::Fill(1), Right(build_ledger_body_ls(ctx)))],
        },
    }
}

fn build_ledger_header_ls(ledger_mode: LedgerMode) -> LayoutSpec {
    let constraints = match ledger_mode {
        LedgerMode::Browse => vec![
            (Constraint::Fill(1), Left(ComponentId::InspectTabs)),
            (Constraint::Fill(1), Left(ComponentId::LedgerModeTabs)),
        ],
        LedgerMode::Search => vec![
            (Constraint::Length(30), Left(ComponentId::InspectTabs)),
            (Constraint::Length(20), Left(ComponentId::LedgerModeTabs)),
            (Constraint::Fill(1), Left(ComponentId::SearchBar)),
        ],
    };
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints,
    }
}

fn build_ledger_body_ls(ctx: &LayoutContext) -> LayoutSpec {
    // Determine the correct details component based on mode
    let details_component = match ctx.ledger_mode {
        LedgerMode::Browse => match ctx.ledger_browse {
            LedgerBrowse::Accounts => ComponentId::LedgerAccountDetails,
            LedgerBrowse::BlockIssuers => ComponentId::LedgerBlockIssuerDetails,
            LedgerBrowse::DReps => ComponentId::LedgerDRepDetails,
            LedgerBrowse::Pools => ComponentId::LedgerPoolDetails,
            LedgerBrowse::Proposals => ComponentId::LedgerProposalDetails,
            LedgerBrowse::Utxos => ComponentId::LedgerUtxoDetails,
        },
        LedgerMode::Search => match ctx.ledger_search {
            LedgerSearch::UtxosByAddress => ComponentId::LedgerUtxosByAddrDetails,
        },
    };

    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (
                Constraint::Percentage(20),
                Right(build_ledger_left_col_ls(ctx)),
            ),
            (Constraint::Percentage(80), Left(details_component)),
        ],
    }
}

fn build_ledger_left_col_ls(ctx: &LayoutContext) -> LayoutSpec {
    let (options_component, list_component) = match ctx.ledger_mode {
        LedgerMode::Browse => {
            let list = match ctx.ledger_browse {
                LedgerBrowse::Accounts => ComponentId::LedgerAccountsList,
                LedgerBrowse::BlockIssuers => ComponentId::LedgerBlockIssuersList,
                LedgerBrowse::DReps => ComponentId::LedgerDRepsList,
                LedgerBrowse::Pools => ComponentId::LedgerPoolsList,
                LedgerBrowse::Proposals => ComponentId::LedgerProposalsList,
                LedgerBrowse::Utxos => ComponentId::LedgerUtxosList,
            };
            (ComponentId::LedgerBrowseOptions, list)
        }
        LedgerMode::Search => {
            let list = match ctx.ledger_search {
                LedgerSearch::UtxosByAddress => ComponentId::LedgerUtxosByAddrList,
            };
            (ComponentId::LedgerSearchOptions, list)
        }
    };

    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Fill(1), Left(options_component)),
            (Constraint::Fill(3), Left(list_component)),
        ],
    }
}

fn build_chain_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![(Constraint::Fill(1), Right(build_chain_rest_ls()))],
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
            (Constraint::Fill(1), Left(ComponentId::InspectTabs)),
            (Constraint::Fill(1), Left(ComponentId::SearchBar)),
        ],
    }
}

fn build_chain_body_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (Constraint::Fill(1), Left(ComponentId::ChainSearchHeader)),
            (Constraint::Fill(1), Left(ComponentId::ChainSearchBlock)),
            (Constraint::Fill(1), Left(ComponentId::ChainSearchNonces)),
        ],
    }
}

fn build_otel_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![(Constraint::Fill(1), Right(build_otel_rest_ls()))],
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
        constraints: vec![(Constraint::Fill(1), Left(ComponentId::InspectTabs))],
    }
}

fn build_otel_body_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (Constraint::Percentage(10), Left(ComponentId::OtelTraceList)),
            (Constraint::Percentage(90), Right(build_otel_details_ls())),
        ],
    }
}

fn build_otel_details_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (
                Constraint::Percentage(70),
                Left(ComponentId::OtelFlameGraph),
            ),
            (
                Constraint::Percentage(30),
                Left(ComponentId::OtelSpanDetails),
            ),
        ],
    }
}

fn build_prom_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![(Constraint::Fill(1), Right(build_prom_rest_ls()))],
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
        constraints: vec![(Constraint::Fill(1), Left(ComponentId::InspectTabs))],
    }
}

fn build_prom_body_ls() -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![(Constraint::Fill(1), Left(ComponentId::PrometheusMetrics))],
    }
}
