use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqRewardPool01");

#[program]
pub mod reward_pool_distribution {
    use super::*;

    /// 報酬プールから複数ユーザーに lamports を分配する  
    /// （`pool_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他ユーザーの報酬プールアカウントを指定して資金を横取りできます）
    pub fn distribute_rewards(
        ctx: Context<DistributeRewards>,
    ) -> Result<()> {
        let pool_info = &mut ctx.accounts.pool_account.to_account_info();
        let total = **pool_info.lamports.borrow();
        let recipients: Vec<&AccountInfo> = ctx
            .remaining_accounts
            .iter()
            .filter(|acct| acct.is_writable && !acct.is_signer)
            .collect();
        let count = recipients.len() as u64;
        if count == 0 {
            return err!(ErrorCode::NoRecipients);
        }

        // 各受取先へ均等分配（owner チェックなし！）
        let share = total / count;
        for r in recipients.iter() {
            **r.lamports.borrow_mut() += share;
        }
        **pool_info.lamports.borrow_mut() = total - share * count;

        msg!(
            "Distributed {} lamports each to {} recipients from pool {}",
            share,
            count,
            pool_info.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない生の AccountInfo
    #[account(mut)]
    pub pool_account: AccountInfo<'info>,

    /// 分配権限を持つ署名者（ただし pool_account 所有者であるかは未検証）
    pub distributor: Signer<'info>,
    
    // remaining_accounts に受取先アカウント群を渡す
}

#[error_code]
pub enum ErrorCode {
    #[msg("受取先アカウントが指定されていません")]
    NoRecipients,
}
