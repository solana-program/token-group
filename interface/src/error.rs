//! Interface error types

use solana_program_error::{ProgramError, ToStr};

/// Errors that may be returned by the interface.
#[repr(u32)]
#[derive(
    Clone,
    Debug,
    Eq,
    thiserror::Error,
    num_derive::FromPrimitive,
    num_enum::TryFromPrimitive,
    PartialEq,
)]
pub enum TokenGroupError {
    /// Size is greater than proposed max size
    #[error("Size is greater than proposed max size")]
    SizeExceedsNewMaxSize = 3_406_457_176,
    /// Size is greater than max size
    #[error("Size is greater than max size")]
    SizeExceedsMaxSize,
    /// Group is immutable
    #[error("Group is immutable")]
    ImmutableGroup,
    /// Incorrect mint authority has signed the instruction
    #[error("Incorrect mint authority has signed the instruction")]
    IncorrectMintAuthority,
    /// Incorrect update authority has signed the instruction
    #[error("Incorrect update authority has signed the instruction")]
    IncorrectUpdateAuthority,
    /// Member account should not be the same as the group account
    #[error("Member account should not be the same as the group account")]
    MemberAccountIsGroupAccount,
}

impl From<TokenGroupError> for ProgramError {
    fn from(e: TokenGroupError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl ToStr for TokenGroupError {
    fn to_str<E>(&self) -> &'static str
    where
        E: 'static + ToStr + TryFrom<u32>,
    {
        match self {
            TokenGroupError::SizeExceedsNewMaxSize => "Size is greater than proposed max size",
            TokenGroupError::SizeExceedsMaxSize => "Size is greater than max size",
            TokenGroupError::ImmutableGroup => "Group is immutable",
            TokenGroupError::IncorrectMintAuthority => {
                "Incorrect mint authority has signed the instruction"
            }
            TokenGroupError::IncorrectUpdateAuthority => {
                "Incorrect update authority has signed the instruction"
            }
            TokenGroupError::MemberAccountIsGroupAccount => {
                "Member account should not be the same as the group account"
            }
        }
    }
}
