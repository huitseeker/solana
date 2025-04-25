use {
    solana_sdk::{hash::Hash, instruction::InstructionError, pubkey::Pubkey},
    thiserror::Error,
};

/// Errors returned by a Core BPF migration.
#[derive(Debug, Error)]
pub enum CoreBpfMigrationError {
    /// Solana instruction error
    #[error("Solana instruction error: {0:?}")]
    InstructionError(#[from] InstructionError),
    /// Bincode serialization error
    #[error("Bincode serialization error: {0:?}")]
    BincodeError(#[from] bincode::Error),
    /// Account not found
    #[error("Account not found: {0:?}")]
    AccountNotFound(Pubkey),
    /// Account exists
    #[error("Account exists: {0:?}")]
    AccountExists(Pubkey),
    /// Incorrect account owner
    #[error("Incorrect account owner for {0:?}")]
    IncorrectOwner(Pubkey),
    /// Program account not executable
    #[error("Program account not executable for program {0:?}")]
    ProgramAccountNotExecutable(Pubkey),
    /// Program has a data account
    #[error("Data account exists for program {0:?}")]
    ProgramHasDataAccount(Pubkey),
    /// Program has no data account
    #[error("Data account does not exist for program {0:?}")]
    ProgramHasNoDataAccount(Pubkey),
    /// Invalid program account
    #[error("Invalid program account: {0:?}")]
    InvalidProgramAccount(Pubkey),
    /// Invalid program data account
    #[error("Invalid program data account: {0:?}")]
    InvalidProgramDataAccount(Pubkey),
    /// Invalid buffer account
    #[error("Invalid buffer account: {0:?}")]
    InvalidBufferAccount(Pubkey),
    /// Arithmetic overflow
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
    /// Upgrade authority mismatch
    #[error("Upgrade authority mismatch. Expected: {0:?}, Got: {1:?}")]
    UpgradeAuthorityMismatch(Pubkey, Option<Pubkey>),
    /// Invalid verified build hash
    #[error("Invalid build hash. Expected: {0:?}, Got: {1:?}")]
    BuildHashMismatch(Hash, Hash),
    /// HTTP GET request failed
    #[error("HTTP GET request failed: {0}")]
    HttpGetRequestFailed(String),
    /// HTTP GET response too large
    #[error("HTTP GET response too large. Max size: {0}, Got: {1}")]
    HttpGetResponseTooLarge(usize, usize),
    /// HTTP GET invalid URL
    #[error("HTTP GET invalid URL: {0}")]
    HttpGetInvalidUrl(String),
}
