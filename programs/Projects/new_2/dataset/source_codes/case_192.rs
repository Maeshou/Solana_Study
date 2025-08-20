use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};

declare_id!("OwnChkD3000000000000000000000000000000004");

#[program]
pub mod token_unfreeze {
    pub fn unfreeze(
        ctx: Context<Unfreeze>,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.frozen_acc;
        // 属性レベルで authority を検証
        acct.is_frozen = false;

        // audit_log は unchecked で記録
        ctx.accounts.audit_log.data.borrow_mut().push(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Unfreeze<'info> {
    #[account(mut, has_one = authority)]
    pub frozen_acc: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    /// CHECK: 監査ログ、所有者検証なし
    #[account(mut)]
    pub audit_log: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
