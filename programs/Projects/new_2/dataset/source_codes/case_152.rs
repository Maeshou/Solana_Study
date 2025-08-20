use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, TokenAccount, Token};

declare_id!("OwnChkEXT00000000000000000000000000000004");

#[program]
pub mod token_burn_ext {
    pub fn burn_tokens_ext(
        ctx: Context<BurnExt>,
        amount: u64,
        reason: String,
    ) -> Result<()> {
        let from_acc = &mut ctx.accounts.from;
        // 所有者検証済み
        from_acc.burned = from_acc.burned.saturating_add(amount);

        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint:      ctx.accounts.mint.to_account_info(),
                from:      from_acc.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        anchor_spl::token::burn(cpi, amount)?;

        // audit_mem は unchecked で詳細ログ
        let mut mem = ctx.accounts.audit_mem.data.borrow_mut();
        mem.extend_from_slice(&amount.to_le_bytes());
        let reason_bytes = reason.as_bytes();
        mem.extend_from_slice(&(reason_bytes.len() as u32).to_le_bytes());
        mem.extend_from_slice(reason_bytes);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnExt<'info> {
    #[account(mut, has_one = authority)]
    pub from: Account<'info, TokenAccountExt>,
    pub authority: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, anchor_spl::token::Mint>,

    /// CHECK: 監査メモリ。所有者検証なし
    #[account(mut)]
    pub audit_mem: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct TokenAccountExt {
    pub owner: Pubkey,
    pub burned: u64,
}
