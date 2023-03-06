use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Hackathon: This account is already an admin")]
    AccountIsAnAdmin,
    #[msg("Hackathon: This account is not an admin")]
    AccountIsNotAnAdmin,
    #[msg("Hackathon: Invalid vote type (single choice is 0, polling is 1)")]
    InvalidVoteType,
    #[msg("Hackathon: The length of title of proposal cannot be longer 100 characters")]
    InvalidTitleLength,
    #[msg("Hackathon: The length of description of proposal cannot be longer 300 characters")]
    InvalidDescriptionLength,
    #[msg("Hackathon: Invalid timestamp")]
    InvalidTimestamp,
    #[msg("Hackathon: The number of options cannot be zero")]
    InvalidNumOfOptions,
    #[msg("Hackathon: The number of tokens per option cannot be zero")]
    InvalidNumOfTokensPerOption,
    #[msg("Hackathon: The threshold cannot be zero or greater than 1000")]
    InvalidThreshold,
    #[msg("Hackathon: The max of options per vote is invalid")]
    InvalidMaxOptionsPerVote,
    #[msg("Hackathon: The option index is invalid")]
    InvalidOption,
    #[msg("Hackathon: The number of options not match with the proposal type")]
    NumberOptionNotMatchProposalType,
    #[msg("Hackathon: The current timestamp cannot vote")]
    NowCannotVote,
    #[msg("Hackathon: The vote is already claimed")]
    IsAlreadyClaimed,
    #[msg("Hackathon: The current timestamp cannot claim")]
    NowCannotClaim,
    #[msg("Hackathon: Nothing to claim")]
    NothingToClaim,
    // #[msg("TreasureStorage: Change type not supported")]
    // NotSupportedChangeType,
    // #[msg("TreasureStorage: Nft type did not supported")]
    // NftTypeNotSupported,
    // #[msg("TreasureStorage: Only operator can do this function")]
    // NotAnOperation,
    // #[msg("TreasureStorage: Invalid system program")]
    // InvalidSystemProgram,
    // #[msg("TreasureStorage: Admin account had created")]
    // AdminAccountHadCreated,
    // #[msg("TreasureStorage: This account is not an operator")]
    // AccountIsNotAnOperator,
    // #[msg("TreasureStorage: This account is an operator")]
    // AccountIsAnOperator,
    // #[msg("TreasureStorage: Not enough land to reduce")]
    // NotEnoughLandToReduce,
    // #[msg("TreasureStorage: Not enough character to reduce")]
    // NotEnoughCharacterToReduce,
}
