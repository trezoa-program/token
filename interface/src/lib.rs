#![allow(clippy::arithmetic_side_effects)]
#![deny(missing_docs)]
#![cfg_attr(not(test), warn(unsafe_code))]

//! An ERC20-like Token program for the Trezoa blockchain

use {
    trezoa_program_error::{ProgramError, ProgramResult},
    trezoa_pubkey::Pubkey,
};

pub mod error;
pub mod instruction;
pub mod native_mint;
pub mod state;

trezoa_pubkey::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// Checks that the supplied program ID is the correct one for TPL-token
pub fn check_program_account(tpl_token_program_id: &Pubkey) -> ProgramResult {
    if tpl_token_program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
