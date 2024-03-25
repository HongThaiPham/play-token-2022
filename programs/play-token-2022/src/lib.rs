pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

declare_id!("97Ub6UZ63pcJBnvavLGG392k35PG51PcF3QJwGbgqt5n");

#[program]
pub mod play_token_2022 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn transfer(ctx: Context<TransferToken2022>, amount: u64) -> Result<()> {
        transfer_token_2022::handler_transfer(ctx, amount)
    }
}
