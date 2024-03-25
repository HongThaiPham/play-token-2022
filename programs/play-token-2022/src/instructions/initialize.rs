use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::rent::ID as RENT_ID;
use anchor_spl::{
    associated_token::{create, AssociatedToken, Create},
    token_2022::{
        spl_token_2022::{
            self,
            extension::{
                metadata_pointer::instruction::initialize as initialize_metadata_pointer,
                transfer_hook::instruction::initialize as intialize_transfer_hook, ExtensionType,
            },
            instruction::{initialize_mint2, AuthorityType},
            state::Mint,
        },
        Token2022,
    },
    token_interface::{mint_to, set_authority, MintTo, SetAuthority},
};
use solana_program::program::{invoke, invoke_signed};
use spl_token_metadata_interface::{
    instruction::initialize as initialize_metadata_account, state::TokenMetadata,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,

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
    pub token_2022_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    /// CHECK: this is fine since we are hard coding the rent sysvar.
    #[account(address = RENT_ID)]
    pub rent: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let size = ExtensionType::try_calculate_account_len::<Mint>(&[
        // ExtensionType::MintCloseAuthority,
        // ExtensionType::PermanentDelegate,
        ExtensionType::MetadataPointer,
        ExtensionType::TransferHook,
    ])
    .unwrap();

    let metadata = TokenMetadata {
        update_authority: spl_pod::optional_keys::OptionalNonZeroPubkey::try_from(Some(
            ctx.accounts.signer.key(),
        ))
        .unwrap(),
        mint: ctx.accounts.mint.key(),
        name: "Leo Token 2022".to_string(),
        symbol: "LT2".to_string(),
        uri: "https://leopham.dev/token-2022/metadata.json".to_string(),
        additional_metadata: vec![],
    };

    let extension_extra_space = metadata.tlv_size_of().unwrap();
    let rent = &Rent::from_account_info(&ctx.accounts.rent.to_account_info())?;
    let lamports = rent.minimum_balance(size + extension_extra_space);
    invoke(
        &solana_program::system_instruction::create_account(
            &ctx.accounts.signer.key(),
            &ctx.accounts.mint.key(),
            lamports,
            (size).try_into().unwrap(),
            &spl_token_2022::id(),
        ),
        &vec![
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.mint.to_account_info(),
        ],
    )?;

    // 2.2: Transfer Hook,
    invoke(
        &intialize_transfer_hook(
            &ctx.accounts.token_2022_program.key(),
            &ctx.accounts.mint.key(),
            Some(ctx.accounts.vault.key()),
            None, // Some(*ctx.program_id), // TO-DO: Add Transfer Hook
        )?,
        &vec![ctx.accounts.mint.to_account_info()],
    )?;

    invoke(
        &initialize_metadata_pointer(
            &ctx.accounts.token_2022_program.key(),
            &ctx.accounts.mint.key(),
            Some(ctx.accounts.vault.key()),
            Some(ctx.accounts.mint.key()),
        )?,
        &vec![ctx.accounts.mint.to_account_info()],
    )?;

    invoke(
        &initialize_mint2(
            &ctx.accounts.token_2022_program.key(),
            &ctx.accounts.mint.key(),
            &ctx.accounts.signer.key(),
            None,
            9,
        )?,
        &vec![ctx.accounts.mint.to_account_info()],
    )?;

    let seeds = &[
        b"vault",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.bumps.vault],
    ];
    let signer_seeds = &[&seeds[..]];

    invoke_signed(
        &initialize_metadata_account(
            &ctx.accounts.token_2022_program.key(),
            &ctx.accounts.mint.key(),
            &ctx.accounts.vault.key(),
            &ctx.accounts.mint.key(),
            &ctx.accounts.signer.key(),
            metadata.name,
            metadata.symbol,
            metadata.uri,
        ),
        &vec![
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.signer.to_account_info(),
        ],
        signer_seeds,
    )?;

    create(CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        Create {
            payer: ctx.accounts.signer.to_account_info(), // payer
            associated_token: ctx.accounts.vault_ata.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(), // owner
            mint: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_2022_program.to_account_info(),
        },
    ))?;

    mint_to(
        CpiContext::new(
            ctx.accounts.token_2022_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault_ata.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ),
        1_000_000_000 * 10u64.pow(9),
    )?;

    set_authority(
        CpiContext::new(
            ctx.accounts.token_2022_program.to_account_info(),
            SetAuthority {
                current_authority: ctx.accounts.signer.to_account_info().clone(),
                account_or_mint: ctx.accounts.mint.to_account_info().clone(),
            },
        ),
        AuthorityType::MintTokens,
        None, // Set mint authority to be None
    )?;
    Ok(())
}
