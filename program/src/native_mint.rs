//! The Mint that represents the native token
#![deprecated(
    since = "8.1.0",
    note = "Use tpl_token_interface::native_mint instead and remove tpl_token as a dependency"
)]
pub use tpl_token_interface::native_mint::*;
