use cosmwasm_std::{CosmosMsg, Empty, Uint128};
use cw_utils::Duration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_core_macros::govmod_query;
use voting::Threshold;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The threshold a proposal must reach to complete.
    pub threshold: Threshold,
    /// The default maximum amount of time a proposal may be voted on
    /// before expiring.
    pub max_voting_period: Duration,
    /// If set to true only members may execute passed
    /// proposals. Otherwise, any address may execute a passed
    /// proposal.
    pub only_members_execute: bool,
    /// Allows changing votes before the proposal expires. If this is
    /// enabled proposals will not be able to complete early as final
    /// vote information is not known until the time of proposal
    /// expiration.
    pub allow_revoting: bool,
    /// Information about the deposit required to create a
    /// proposal. None if there is no deposit requirement, Some
    /// otherwise.
    pub deposit_info: Option<DepositInfo>,
}

/// Information about the token to use for proposal deposits.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DepositToken {
    /// Use a specific token address as the deposit token.
    Token { address: String },
    /// Use the token address of the associated DAO's voting
    /// module. NOTE: in order to use the token address of the voting
    /// module the voting module must (1) use a cw20 token and (2)
    /// implement the `TokenContract {}` query type defined by
    /// `cw_core_macros::token_query`. Failing to implement that
    /// and using this option will cause instantiation to fail.
    VotingModuleToken {},
}

/// Information about the deposit required to create a proposal.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DepositInfo {
    /// The address of the cw20 token to be used for proposal
    /// deposits.
    pub token: DepositToken,
    /// The number of tokens that must be deposited to create a
    /// proposal.
    pub deposit: Uint128,
    /// If failed proposals should have their deposits refunded.
    pub refund_failed_proposals: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Creates a proposal in the governance module.
    Propose {
        /// The title of the proposal.
        title: String,
        /// A description of the proposal.
        description: String,
        /// The messages that should be executed in response to this
        /// proposal passing.
        msgs: Vec<CosmosMsg<Empty>>,
    },
    /// Votes on a proposal. Voting power is determined by the DAO's
    /// voting power module.
    Vote {
        /// The ID of the proposal to vote on.
        proposal_id: u64,
        /// The senders position on the proposal.
        vote: voting::Vote,
    },
    /// Causes the messages associated with a passed proposal to be
    /// executed by the DAO.
    Execute {
        /// The ID of the proposal to execute.
        proposal_id: u64,
    },
    /// Closes a proposal that has failed (either not passed or timed
    /// out). If applicable this will cause the proposal deposit
    /// associated wth said proposal to be returned.
    Close {
        /// The ID of the proposal to close.
        proposal_id: u64,
    },
    /// Updates the governance module's config.
    UpdateConfig {
        /// The new proposal passing threshold. This will only apply
        /// to proposals created after the config update.
        threshold: Threshold,
        /// The default maximum amount of time a proposal may be voted
        /// on before expiring. This will only apply to proposals
        /// created after the config update.
        max_voting_period: Duration,
        /// If set to true only members may execute passed
        /// proposals. Otherwise, any address may execute a passed
        /// proposal. Applies to all outstanding and future proposals.
        only_members_execute: bool,
        /// Allows changing votes before the proposal expires. If this is
        /// enabled proposals will not be able to complete early as final
        /// vote information is not known until the time of proposal
        /// expiration.
        allow_revoting: bool,
        /// The address if tge DAO that this governance module is
        /// associated with.
        dao: String,
        /// Information about the deposit required to make a
        /// proposal. None if no deposit, Some otherwise.
        deposit_info: Option<DepositInfo>,
    },
    AddProposalHook {
        address: String,
    },
    RemoveProposalHook {
        address: String,
    },
    AddVoteHook {
        address: String,
    },
    RemoveVoteHook {
        address: String,
    },
}

#[govmod_query]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Gets the governance module's config. Returns `state::Config`.
    Config {},
    /// Gets information about a proposal. Returns
    /// `proposals::Proposal`.
    Proposal {
        proposal_id: u64,
    },
    ListProposals {
        start_after: Option<u64>,
        limit: Option<u64>,
    },
    ReverseProposals {
        start_before: Option<u64>,
        limit: Option<u64>,
    },
    ProposalCount {},
    Vote {
        proposal_id: u64,
        voter: String,
    },
    ListVotes {
        proposal_id: u64,
        start_after: Option<String>,
        limit: Option<u64>,
    },
    ProposalHooks {},
    VoteHooks {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}
