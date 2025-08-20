use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqEmergWthDr01");

#[program]
pub mod emergency_withdrawal {
    use super::*;

    /// 緊急時にプールアカウントから指定アカウントへ lamports を引き出す  
    /// （`pool_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人の資産プールを指定して好き勝手に引き出せる脆弱性があります）
    pub fn emergency_withdraw(
        ctx: Context<EmergencyWithdraw>,
        amount: u64,  // 引き出す lamports
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool_account.to_account_info();
        let dest = &mut ctx.accounts.destination.to_account_info();

        // ★ owner == program_id の検証を省略！
        let pool_balance = **pool.lamports.borrow();
        if pool_balance < amount {
            return err!(ErrorCode::InsufficientFunds);
        }

        // lamports を移動
        **pool.lamports.borrow_mut() -= amount;
        **dest.lamports.borrow_mut() += amount;

        msg!(
            "Emergency withdrew {} lamports from pool {} to {}",
            amount,
            pool.key(),
            dest.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EmergencyWithdraw<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub pool_account: AccountInfo<'info>,

    /// lamports を受け取るアカウント（owner チェックなし）
    #[account(mut)]
    pub destination:  AccountInfo<'info>,

    /// 緊急引き出しを実行する権限があると仮定した署名者
    pub authority:   Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("プールに十分な資金がありません")]
    InsufficientFunds,
}
