use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("RewdVar0333333333333333333333333333333333");

#[program]
pub mod reward_var3 {
    pub fn distribute(ctx: Context<Distribute>) -> Result<()> {
        let rd = &mut ctx.accounts.reward;
        // 手動 if で管理者チェック
        if ctx.accounts.admin.key() != rd.admin {
            return Err(ProgramError::Custom(3).into());
        }
        // 各ユーザーに均等配布
        let per = rd.total / rd.recipients.len() as u64;
        for r in rd.recipients.iter() {
            *rd.paid.entry(*r).or_insert(0) += per;
        }

        // refund_acc は unchecked で lamports 調整可能
        **ctx.accounts.refund_acc.lamports.borrow_mut() += per;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut)]
    pub reward: Account<'info, RewardData>,
    pub admin: Signer<'info>,
    #[account(mut)] pub refund_acc: AccountInfo<'info>,  // unchecked
}

#[account]
pub struct RewardData {
    pub admin: Pubkey,
    pub total: u64,
    pub recipients: Vec<Pubkey>,
    pub paid: BTreeMap<Pubkey, u64>,
}
