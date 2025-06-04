use crate::{
    components::list_and_details::proposal::ProposalItem,
    to_rich::{
        RationalNumberDisplay, ToRichText, labeled_default, labeled_default_opt,
        labeled_default_opt_single, labeled_default_single,
    },
};
use amaru_kernel::{
    CostModel, CostModels, ExUnitPrices, ExUnits, GovAction, PoolVotingThresholds, Proposal,
    ProposalId, ProposalPointer, ProtocolParamUpdate, TransactionPointer,
};
use amaru_ledger::store::columns::proposals;
use ratatui::text::Span;
use std::fmt;

#[derive(Clone)]
pub struct ProposalIdDisplay<'a>(pub &'a ProposalId);

impl<'a> fmt::Display for ProposalIdDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0.transaction_id, self.0.action_index)
    }
}

impl ToRichText for ProposalId {
    fn to_rich_text(&self) -> super::RichText {
        super::RichText::Single(Span::raw(ProposalIdDisplay(self).to_string()))
    }
}

impl ToRichText for ProposalItem {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single(
            "Proposal",
            ProposalIdDisplay(&self.0),
        ));
        lines.extend(self.1.to_rich_text().unwrap_lines());
        super::RichText::Lines(lines)
    }
}

impl ToRichText for proposals::Row {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default("Proposed In", &self.proposed_in));
        lines.extend(labeled_default_single("Valid Until", self.valid_until));
        lines.extend(labeled_default("Proposal", &self.proposal));
        super::RichText::Lines(lines)
    }
}

impl ToRichText for ProposalPointer {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default("Transaction", &self.transaction));
        lines.extend(labeled_default_single(
            "Proposal Index",
            self.proposal_index,
        ));
        super::RichText::Lines(lines)
    }
}

impl ToRichText for TransactionPointer {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single("Slot", self.slot));
        lines.extend(labeled_default_single(
            "Transaction Index",
            self.transaction_index,
        ));
        super::RichText::Lines(lines)
    }
}

impl ToRichText for Proposal {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single("Deposit", self.deposit));
        lines.extend(labeled_default_single(
            "Reward Account",
            self.reward_account.to_string(),
        ));
        lines.extend(labeled_default("Gov Action", &self.gov_action));
        lines.extend(labeled_default("Anchor", &self.anchor));
        super::RichText::Lines(lines)
    }
}

impl ToRichText for GovAction {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = vec![];

        match self {
            GovAction::ParameterChange(nid, pu, nsh) => {
                lines.extend(labeled_default_single("Type", "ParameterChange"));
                lines.extend(labeled_default("Gov Action Id", nid));
                lines.extend(labeled_default("Protocol Param Update", pu.as_ref()));
                lines.extend(labeled_default("Script Hash", nsh));
            }
            GovAction::HardForkInitiation(nid, version) => {
                lines.extend(labeled_default_single("Type", "HardForkInitiation"));
                lines.extend(labeled_default("Gov Action Id", nid));
                lines.extend(labeled_default("Protocol Version", version));
            }
            GovAction::TreasuryWithdrawals(withdrawals, nsh) => {
                lines.extend(labeled_default_single("Type", "TreasuryWithdrawals"));
                lines.extend(labeled_default("Withdrawals", withdrawals));
                lines.extend(labeled_default("Script Hash", nsh));
            }
            GovAction::NoConfidence(nid) => {
                lines.extend(labeled_default_single("Type", "NoConfidence"));
                lines.extend(labeled_default("Gov Action Id", nid));
            }
            GovAction::UpdateCommittee(nid, cccs, ccckvps, ui) => {
                lines.extend(labeled_default_single("Type", "UpdateCommittee"));
                lines.extend(labeled_default("Gov Action Id", nid));
                lines.extend(labeled_default("Committee Cold Credential Set", cccs));
                lines.extend(labeled_default(
                    "CommitteeColdCredential, Epoch Pairs",
                    ccckvps,
                ));
                lines.extend(labeled_default("Unit Interval", ui));
            }
            GovAction::NewConstitution(nid, constitution) => {
                lines.extend(labeled_default_single("Type", "NewConstitution"));
                lines.extend(labeled_default("Gov Action Id", nid));
                lines.extend(labeled_default_single(
                    "Constitution",
                    format!("{:?}", constitution),
                ));
            }
            GovAction::Information => {
                lines.extend(labeled_default_single("Type", "Information"));
            }
        }

        super::RichText::Lines(lines)
    }
}

impl ToRichText for ProtocolParamUpdate {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = Vec::new();
        //         pub struct ProtocolParamUpdate {
        lines.extend(labeled_default_opt_single("Min Fee {a}", self.minfee_a));
        lines.extend(labeled_default_opt_single("Min Fee {b}", self.minfee_b));
        lines.extend(labeled_default_opt_single(
            "Max block body size",
            self.max_block_body_size,
        ));
        lines.extend(labeled_default_opt_single(
            "Max block header size",
            self.max_block_header_size,
        ));
        lines.extend(labeled_default_opt_single("Key deposit", self.key_deposit));
        lines.extend(labeled_default_opt_single(
            "Pool deposit",
            self.pool_deposit,
        ));
        lines.extend(labeled_default_opt_single("Max epoch", self.maximum_epoch));
        lines.extend(labeled_default_opt_single(
            "Desired # of stake pools",
            self.desired_number_of_stake_pools,
        ));
        lines.extend(labeled_default_opt_single(
            "Pool pledge influence",
            self.pool_pledge_influence
                .as_ref()
                .map(RationalNumberDisplay),
        ));
        lines.extend(labeled_default_opt_single(
            "Expansion rate",
            self.expansion_rate.as_ref().map(RationalNumberDisplay),
        ));
        lines.extend(labeled_default_opt_single(
            "Treasury growth rate",
            self.treasury_growth_rate
                .as_ref()
                .map(RationalNumberDisplay),
        ));
        lines.extend(labeled_default_opt_single(
            "Min pool cost",
            self.min_pool_cost,
        ));
        lines.extend(labeled_default_opt_single(
            "ADA per UTXO byte",
            self.ada_per_utxo_byte,
        ));
        lines.extend(labeled_default_opt(
            "Cost models for script languages",
            self.cost_models_for_script_languages.as_ref(),
        ));
        lines.extend(labeled_default_opt(
            "Execution costs",
            self.execution_costs.as_ref(),
        ));
        lines.extend(labeled_default_opt(
            "Max tx ex units",
            self.max_tx_ex_units.as_ref(),
        ));
        lines.extend(labeled_default_opt(
            "Max block ex units",
            self.max_block_ex_units.as_ref(),
        ));
        lines.extend(labeled_default_opt_single(
            "Max value size",
            self.max_value_size,
        ));
        lines.extend(labeled_default_opt_single(
            "Collateral percentage",
            self.collateral_percentage,
        ));
        lines.extend(labeled_default_opt_single(
            "Max collateral inputs",
            self.max_collateral_inputs,
        ));
        lines.extend(labeled_default_opt(
            "Pool voting thresholds",
            self.pool_voting_thresholds.as_ref(),
        ));
        lines.extend(labeled_default_opt_single(
            "Min committee size",
            self.min_committee_size,
        ));
        lines.extend(labeled_default_opt_single(
            "Committee term limit",
            self.committee_term_limit,
        ));
        lines.extend(labeled_default_opt_single(
            "Governance action validity period",
            self.governance_action_validity_period,
        ));
        lines.extend(labeled_default_opt_single(
            "Governance action deposit",
            self.governance_action_deposit,
        ));
        lines.extend(labeled_default_opt_single(
            "DRep deposit",
            self.drep_deposit,
        ));
        lines.extend(labeled_default_opt_single(
            "DRep inactivity period",
            self.drep_inactivity_period,
        ));
        lines.extend(labeled_default_opt_single(
            "Min fee ref script cost per byte",
            self.minfee_refscript_cost_per_byte
                .as_ref()
                .map(RationalNumberDisplay),
        ));
        super::RichText::Lines(lines)
    }
}

impl ToRichText for CostModels {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_opt_single(
            "Plutus V1",
            self.plutus_v1.as_ref().map(CostModelDisplay),
        ));
        lines.extend(labeled_default_opt_single(
            "Plutus V2",
            self.plutus_v2.as_ref().map(CostModelDisplay),
        ));
        lines.extend(labeled_default_opt_single(
            "Plutus V3",
            self.plutus_v3.as_ref().map(CostModelDisplay),
        ));
        super::RichText::Lines(lines)
    }
}

pub struct CostModelDisplay<'a>(pub &'a CostModel);

impl<'a> fmt::Display for CostModelDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl ToRichText for ExUnitPrices {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single(
            "Mem price",
            RationalNumberDisplay(&self.mem_price),
        ));
        lines.extend(labeled_default_single(
            "Step price",
            RationalNumberDisplay(&self.step_price),
        ));
        super::RichText::Lines(lines)
    }
}

impl ToRichText for ExUnits {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single("Mem", self.mem));
        lines.extend(labeled_default_single("Steps", self.steps));
        super::RichText::Lines(lines)
    }
}

impl ToRichText for PoolVotingThresholds {
    fn to_rich_text(&self) -> super::RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default(
            "Motion No Confidence",
            &self.motion_no_confidence,
        ));
        lines.extend(labeled_default("Committee Normal", &self.committee_normal));
        lines.extend(labeled_default(
            "Committee No Confidence",
            &self.committee_no_confidence,
        ));
        lines.extend(labeled_default(
            "Hard Fork Initiation",
            &self.hard_fork_initiation,
        ));
        lines.extend(labeled_default(
            "Security Voting Threshold",
            &self.security_voting_threshold,
        ));
        super::RichText::Lines(lines)
    }
}
