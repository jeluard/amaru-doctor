use crate::{
    components::r#static::{entity_types::Entity, search_types::Search},
    shared::{Shared, shared},
    store::{
        owned_iter::{
            OwnedAccountsIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter,
            OwnedProposalIter, OwnedUtxoIter,
        },
        rocks_db_switch::RocksDBSwitch,
    },
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
    window::WindowState,
};
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct AppModel {
    pub entity_list: Shared<WindowState<Entity>>,
    pub search_list: Shared<WindowState<Search>>,
    pub account_list: Shared<WindowState<AccountItem>>,
    pub block_issuer_list: Shared<WindowState<BlockIssuerItem>>,
    pub drep_list: Shared<WindowState<DRepItem>>,
    pub pool_list: Shared<WindowState<PoolItem>>,
    pub proposal_list: Shared<WindowState<ProposalItem>>,
    pub utxo_list: Shared<WindowState<UtxoItem>>,
}

impl AppModel {
    pub fn new(db: Arc<RocksDBSwitch>) -> Self {
        Self {
            entity_list: shared(WindowState::new(Box::new(Entity::iter()))),
            search_list: shared(WindowState::new(Box::new(Search::iter()))),
            account_list: shared(WindowState::new(Box::new(OwnedAccountsIter::new(
                db.clone(),
            )))),
            block_issuer_list: shared(WindowState::new(Box::new(OwnedBlockIssuerIter::new(
                db.clone(),
            )))),
            drep_list: shared(WindowState::new(Box::new(OwnedDRepIter::new(db.clone())))),
            pool_list: shared(WindowState::new(Box::new(OwnedPoolIter::new(db.clone())))),
            proposal_list: shared(WindowState::new(Box::new(OwnedProposalIter::new(
                db.clone(),
            )))),
            utxo_list: shared(WindowState::new(Box::new(OwnedUtxoIter::new(db.clone())))),
        }
    }

    // fn get_list<T>(&self, entity: Entity) -> WindowState<T> {
    //     match entity {
    //         Entity::Accounts => self.account_list,
    //         Entity::BlockIssuers => self.block_issuers_list,
    //         Entity::DReps => self.drep_list,
    //         Entity::Pools => self.pool_list,
    //         Entity::Proposals => self.proposal_list,
    //         Entity::UTXOs => self.utxo_list,
    //         Entity::Entites => panic!("DNE"),
    //         Entity::SearchTypes => panic!("DNE"),
    //     }
    // }
}
