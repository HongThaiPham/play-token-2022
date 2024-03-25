use anchor_lang::{prelude::*, solana_program::sysvar::rent::ID as RENT_ID};
use anchor_spl::{
    associated_token::{create, AssociatedToken, Create},
    token_2022::Token2022,
    token_interface::{transfer_checked, TransferChecked},
};

#[derive(Accounts)]
pub struct TransferToken2022<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK:
    #[account(
        mut,
        seeds = [b"vault", mint.key().as_ref()],
        bump
    )]
    pub vault: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            vault.key().as_ref(),
            token_2022_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = associated_token_program.key(),
        bump
    )]
    /// CHECK
    pub vault_ata: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            signer.key().as_ref(),
            token_2022_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = associated_token_program.key(),
        bump
    )]
    /// CHECK
    pub user_ata: UncheckedAccount<'info>,
    pub token_2022_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    /// CHECK: this is fine since we are hard coding the rent sysvar.
    #[account(address = RENT_ID)]
    pub rent: UncheckedAccount<'info>,
}

pub fn handler_transfer(ctx: Context<TransferToken2022>, amount: u64) -> Result<()> {
    let seeds = &[
        b"vault",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.bumps.vault],
    ];
    let signer_seeds = &[&seeds[..]];

    create(CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        Create {
            payer: ctx.accounts.signer.to_account_info(), // payer
            associated_token: ctx.accounts.user_ata.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(), // owner
            mint: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_2022_program.to_account_info(),
        },
    ))?;

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_2022_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_ata.to_account_info(),
                to: ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        9,
    )?;
    Ok(())
}
