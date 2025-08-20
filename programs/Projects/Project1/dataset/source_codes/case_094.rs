use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfPENDLT");

#[program]
pub mod penalty_distributor {
    use super::*;

    /// ペナルティプールから正常ユーザーへ一律分配
    pub fn distribute_penalties(ctx: Context<DistributePenalties>) -> Result<()> {
        require!(ctx.accounts.admin.is_signer, ErrorCode::Unauthorized);

        let vault_info = ctx.accounts.vault.to_account_info();
        let mut vault_lamports = **vault_info.lamports.borrow();
        let recipients = &ctx.remaining_accounts;
        let count = recipients.len() as u64;
        require!(count > 0, ErrorCode::NoRecipients);

        // 1人あたりの分配量
        let share = vault_lamports / count;

        // 各受取人へ送金
        for recipient_info in recipients.iter() {
            **vault_info.try_borrow_mut_lamports()? -= share;
            **recipient_info.try_borrow_mut_lamports()? += share;
            vault_lamports -= share;
            msg!(
                "Distributed {} lamports to {}",
                share,
                recipient_info.key
            );
        }

        // 残高をログに出力
        msg!("Remaining in vault: {}", vault_lamports);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributePenalties<'info> {
    /// 罰金トークン保管用PDA
    #[account(mut, seeds = [b"penalty_vault"], bump)]
    pub vault: SystemAccount<'info>,

    /// 管理者（分配実行者）
    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,

    // 残りのアカウントは受取人として扱われます
    // Anchorではremaining_accountsでアクセス可能
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: admin signature required")]
    Unauthorized,
    #[msg("No recipients provided")]
    NoRecipients,
}
