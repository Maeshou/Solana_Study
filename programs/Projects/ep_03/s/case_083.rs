use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgArenaSvc03");

#[program]
pub mod arena_service {
    use super::*;

    /// アリーナに参加登録し、参加料を徴収するが、
    /// entry.owner と ctx.accounts.user.key() の一致検証がない
    pub fn join_arena(ctx: Context<ModifyArena>, entry_fee: u64) -> Result<()> {
        let entry = &mut ctx.accounts.arena_entry;
        // 1. 参加料をプールに移動
        collect_fee(&ctx.accounts.user, &ctx.accounts.treasury, entry_fee)?;
        // 2. 登録回数をインクリメント
        entry.join_count = entry.join_count.checked_add(1).unwrap();
        // 3. 最後に参加したアリーナIDを設定
        entry.last_arena = ctx.accounts.config.arena_id;
        Ok(())
    }

    /// アリーナから退会し、（一部）返金するが、
    /// entry.owner と ctx.accounts.user.key() の一致検証がない
    pub fn leave_arena(ctx: Context<ModifyArena>, refund_amount: u64) -> Result<()> {
        let entry = &mut ctx.accounts.arena_entry;
        // 1. 退会回数をインクリメント
        entry.leave_count = entry.leave_count.checked_add(1).unwrap();
        // 2. 一部返金を実行
        refund_fee(&ctx.accounts.treasury, &ctx.accounts.user, refund_amount)?;
        Ok(())
    }
}

/// 直接 lamports を操作して参加料をプールに移動するヘルパー
fn collect_fee(from: &AccountInfo, to: &AccountInfo, amount: u64) -> Result<()> {
    **from.lamports.borrow_mut() -= amount;
    **to.lamports.borrow_mut() += amount;
    Ok(())
}

/// 直接 lamports を操作して返金を行うヘルパー
fn refund_fee(from: &AccountInfo, to: &AccountInfo, amount: u64) -> Result<()> {
    **from.lamports.borrow_mut() -= amount;
    **to.lamports.borrow_mut() += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct ModifyArena<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub arena_entry: Account<'info, ArenaEntry>,
    /// 参加料／返金用プール
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    /// 操作をリクエストするユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,
    /// アリーナ設定（IDなど）を保持するアカウント
    pub config: Account<'info, ArenaConfig>,
}

#[account]
pub struct ArenaEntry {
    /// 本来この登録を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 参加登録回数
    pub join_count: u64,
    /// 退会回数
    pub leave_count: u64,
    /// 最後に参加したアリーナの ID
    pub last_arena: u64,
}

#[account]
pub struct ArenaConfig {
    /// 固定のアリーナ識別子
    pub arena_id: u64,
}
