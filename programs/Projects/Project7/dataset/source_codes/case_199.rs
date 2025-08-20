use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use spl_token::instruction as spl_ix;
use spl_token::ID as SPL_TOKEN_ID;

declare_id!("SaFeMinTToCPI11111111111111111111111111");

#[program]
pub mod safe_token_mint {
    use super::*;

    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        // 実行先は spl_token の固定ID。直前に鍵検証も実施。
        require_keys_eq!(ctx.accounts.token_program.key(), SPL_TOKEN_ID);

        let ix = spl_ix::mint_to(
            &SPL_TOKEN_ID,
            ctx.accounts.mint.key,
            ctx.accounts.dest.key,
            ctx.accounts.mint_authority.key,
            &[],
            amount,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.dest.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Mint<'info> {
    /// CHECK:
    pub token_program: AccountInfo<'info>,
    /// CHECK:
    pub mint: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub dest: AccountInfo<'info>,
    /// CHECK:
    pub mint_authority: AccountInfo<'info>,
}
