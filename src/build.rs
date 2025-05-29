use crate::{
    action::SelectedItem,
    components::{
        details::DetailsComponent,
        fps::FpsCounter,
        group::ComponentGroup,
        layout::RootLayout,
        message::Message,
        resources::ResourceList,
        scroll::ScrollableListComponent,
        split::{Axis, SplitComponent},
        utxo::new_utxo_list_component,
    },
    focus::FocusManager,
    shared::shared,
    to_rich::ToRichText,
};
use amaru_ledger::store::ReadOnlyStore;
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use std::sync::Arc;

pub fn build_layout<'a>(
    ledger_path_str: &String,
    db: &'a Arc<RocksDB>,
) -> Result<(RootLayout<'a>, FocusManager<'a>)> {
    let resource_list = shared(ResourceList::default());
    let utxos = shared(new_utxo_list_component(db));
    let utxo_details = shared(DetailsComponent::new(
        "UTXO Details".to_string(),
        |s| match s {
            SelectedItem::Utxo(k) => Some(k.clone()),
            _ => None,
        },
        move |key| {
            let val = db.utxo(key)?;
            Ok(val.map(|v| (key.clone(), v).into_rich_text().unwrap_lines()))
        },
    ));

    let body = shared(SplitComponent::new_2(
        Axis::Vertical,
        30,
        shared(SplitComponent::new_2_evenly(
            Axis::Horizontal,
            resource_list.clone(),
            utxos.clone(),
        )),
        70,
        utxo_details.clone(),
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
            "Use Shift + Left/Right/Up/Down (←↑→↓) to move focus. Use Left/Right/Up/Down to scroll within focus.".to_string(),
        ))])),
    );

    Ok((
        layout,
        FocusManager::new(vec![
            utxos.clone(),
            utxo_details.clone(),
            resource_list.clone(),
        ]),
    ))
}
