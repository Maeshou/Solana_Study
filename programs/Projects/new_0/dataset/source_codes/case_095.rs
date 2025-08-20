use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfEQ2USR");

#[program]
pub mod penalty_splitter {
    use super::*;

    /// ペナルティプールから二名の正常ユーザーへ均等分配
    pub fn split_penalty(ctx: Context<SplitPenalty>) -> Result<()> {
        // 管理者署名を必須化
        require!(ctx.accounts.admin.is_signer, ErrorCode::Unauthorized);

        // プール残高取得
        let vault_info = ctx.accounts.vault.to_account_info();
        let total = **vault_info.lamports.borrow();

        // 二分割
        let half = total
            .checked_div(2)
            .unwrap(); // 安全に半分に分ける

        // 受取人Aへ送金
        **vault_info.try_borrow_mut_lamports()? -= half;
        **ctx.accounts.user_a.to_account_info().try_borrow_mut_lamports()? += half;
        msg!(
            "Sent {} lamports to {}",
            half,
            ctx.accounts.user_a.key()
        );

        // 受取人Bへ送金（残った分を送金）
        let remainder = **vault_info.lamports.borrow();
        **vault_info.try_borrow_mut_lamports()? -= remainder;
        **ctx.accounts.user_b.to_account_info().try_borrow_mut_lamports()? += remainder;
        msg!(
            "Sent {} lamports to {}",
            remainder,
            ctx.accounts.user_b.key()
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SplitPenalty<'info> {
    /// 罰金トークン保管用PDA
    #[account(mut, seeds = [b"penalty_vault"], bump)]
    pub vault: SystemAccount<'info>,

    /// 分配実行管理者
    #[account(mut)]
    pub admin: Signer<'info>,

    /// 正常ユーザーA（署名不要）
    #[account(mut)]
    pub user_a: SystemAccount<'info>,

    /// 正常ユーザーB（署名不要）
    #[account(mut)]
    pub user_b: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Admin must sign transaction")]
    Unauthorized,
}
