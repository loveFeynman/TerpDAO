use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized.")]
    Unauthorized {},

    #[error("DAO does not have an admin configured.")]
    NoAdmin {},

    #[error("The contract is paused.")]
    Paused {},

    #[error("No voting module provided.")]
    NoVotingModule {},

    #[error("Execution would result in no governance modules being present.")]
    NoProposalModule {},

    #[error("An unknown reply ID was received.")]
    UnknownReplyID {},

    #[error("{0}")]
    ParseReplyError(#[from] ParseReplyError),

    #[error("Multiple voting modules during instantiation.")]
    MultipleVotingModules {},

    #[error("Unsigned integer overflow.")]
    Overflow {},

    #[error("You can only instantiate {0} items during instantiation, but you tried to instantiate {1}.")]
    TooManyItems(u64, usize),

    #[error("Key is missing from storage")]
    KeyMissing {},
}
