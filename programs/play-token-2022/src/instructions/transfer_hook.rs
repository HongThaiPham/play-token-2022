use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::{
    extension::{transfer_hook::TransferHookAccount, BaseStateWithExtensions, StateWithExtensions},
    state::Account as Token2022Account,
};
use spl_transfer_hook_interface::error::TransferHookError;

fn check_token_account_is_transferring(account_data: &[u8]) -> Result<()> {
    let token_account = StateWithExtensions::<Token2022Account>::unpack(account_data)?;
    let extension = token_account.get_extension::<TransferHookAccount>()?;
    if bool::from(extension.transferring) {
        Ok(())
    } else {
        Err(Into::<ProgramError>::into(
            TransferHookError::ProgramCalledOutsideOfTransfer,
        ))?
    }
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferHook<'info> {
    /// CHECK:
    pub source: AccountInfo<'info>,
    /// CHECK:
    pub mint: AccountInfo<'info>,
    /// CHECK:
    pub destination: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    /// CHECK: must be the extra account PDA
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()], 
        bump)
    ]
    pub extra_account: AccountInfo<'info>,
}

impl<'info> TransferHook<'info> {
    pub fn hanlder<'a>(&mut self, amount: u64) -> Result<()> {
        let source_account = self.source.to_account_info();
        let destination_account = self.destination.to_account_info();

        check_token_account_is_transferring(&source_account.to_account_info().try_borrow_data()?)?;
        check_token_account_is_transferring(
            &destination_account.to_account_info().try_borrow_data()?,
        )?;

        msg!("Transfer hook invoked");
        msg!("Transfer amount: {}", amount);
        msg!(
            "Transfer with extra account PDA: {}",
            self.extra_account.key()
        );
        Ok(())
    }
}
