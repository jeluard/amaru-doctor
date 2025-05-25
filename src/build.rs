use std::sync::Arc;

use amaru_ledger::store::ReadOnlyStore;
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use ratatui::widgets::ListItem;

use crate::{
    components::{
        Component,
        empty::EmptyComponent,
        fps::FpsCounter,
        group::ComponentGroup,
        layout::RootLayout,
        message::Message,
        resources::ResourceList,
        scroll::ScrollableListComponent,
        split::{Axis, SplitComponent},
    },
    focus::{FocusManager, Focusable},
    shared::{Shared, shared},
};

pub struct LayoutAndFocus<'a> {
    pub layout: RootLayout<'a>,
    pub focus: FocusManager<'a>,
}

pub fn build_layout<'a>(
    ledger_path_str: &String,
    db: &'a Arc<RocksDB>,
) -> Result<(RootLayout<'a>, FocusManager<'a>)> {
    let resource_list = shared(ResourceList::default());
    let utxos = shared(ScrollableListComponent::new(
        "UTXOs".to_string(),
        db.iter_utxos()?.enumerate(),
        10,
        |(i, (input, _))| ListItem::new(format!("{}: {}", i, input.transaction_id.to_string())),
    ));
    let details = shared(EmptyComponent::default());

    let body = shared(SplitComponent::new_2(
        Axis::Vertical,
        30,
        shared(SplitComponent::new_2_evenly(
            Axis::Horizontal,
            resource_list.clone(),
            utxos.clone(),
        )),
        70,
        details.clone(),
    ));

    let layout = RootLayout::new(
        shared(ComponentGroup::new(vec![
            shared(Message::new(format!(
                "Reading amaru ledger at {:?}",
                ledger_path_str
            ))),
            shared(FpsCounter::default()),
        ])),
        body,
        shared(ComponentGroup::new(vec![shared(Message::new(
            "Use arrow keys ←↑→↓ to navigate.".to_string(),
        ))])),
    );

    Ok((
        layout,
        FocusManager::new(vec![resource_list.clone(), utxos.clone(), details.clone()]),
    ))
}
