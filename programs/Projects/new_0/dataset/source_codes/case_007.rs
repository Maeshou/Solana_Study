use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf888mvTWf");

#[program]
pub mod event_logger_003 {
    use super::*;

    // イベントログとして履歴を蓄積（今回は件数だけ記録）
    pub fn log_event(ctx: Context<Ctx003>, event_name: String) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);

        ctx.accounts.storage.data = ctx.accounts.storage.data.checked_add(1).unwrap();
        msg!(
            "Logged Event [{}] from {} | Count: {}",
            event_name,
            ctx.accounts.authority.key(),
            ctx.accounts.storage.data
        );
        Ok(())
    }

    // 履歴を初期化
    pub fn clear_history(ctx: Context<Ctx003>) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);
        ctx.accounts.storage.data = 0;
        msg!("Event history cleared by {}", ctx.accounts.authority.key());
        Ok(())
    }

    // 現在の履歴件数が指定上限を超えているかどうかチェック
    pub fn check_event_limit(ctx: Context<Ctx003>, limit: u64) -> Result<()> {
        let count = ctx.accounts.storage.data;
        let over = count > limit;
        msg!(
            "Event Count = {} | Limit = {} | Over Limit = {}",
            count,
            limit,
            over
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub data: u64, // イベント件数カウンタとして利用
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
}
