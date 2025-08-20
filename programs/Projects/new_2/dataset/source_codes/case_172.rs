use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, TokenAccount, Token};

declare_id!("OwnChkC4000000000000000000000000000000004");

#[program]
pub mod nft_burn {
    pub fn burn_nft(
        ctx: Context<BurnNft>,
        amount: u64,
    ) -> Result<()> {
        // 属性レベルで owner を検証
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint:      ctx.accounts.mint.to_account_info(),
                from:      ctx.accounts.from.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        );
        anchor_spl::token::burn(cpi, amount)?;
        // audit_log は unchecked
        ctx.accounts.audit_log.data.borrow_mut().push(amount as u8);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnNft<'info> {
    #[account(mut, has_one = owner)]
    pub from: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, anchor_spl::token::Mint>,
    /// CHECK: 監査ログ、所有者検証なし
    #[account(mut)]
    pub audit_log: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
