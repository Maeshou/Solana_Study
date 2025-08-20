use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBlockUsr01");

#[program]
pub mod blocklist_service {
    use super::*;

    /// ユーザーをブロックリストに追加するが、
    /// blocklist_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn block_user(ctx: Context<ModifyBlocklist>, target: Pubkey) -> Result<()> {
        let blk = &mut ctx.accounts.blocklist_account;
        record_block(blk, target);
        Ok(())
    }

    /// ユーザーをブロックリストから削除するが、
    /// blocklist_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn unblock_user(ctx: Context<ModifyBlocklist>, target: Pubkey) -> Result<()> {
        let blk = &mut ctx.accounts.blocklist_account;
        record_unblock(blk, target);
        Ok(())
    }
}

/// ブロックリストへ追加し、カウンタを更新するヘルパー関数
fn record_block(blk: &mut BlocklistAccount, target: Pubkey) {
    if !blk.blocked.contains(&target) {
        blk.blocked.push(target);
        blk.block_count = blk.block_count.saturating_add(1);
    }
}

/// ブロックリストから削除し、カウンタを更新するヘルパー関数
fn record_unblock(blk: &mut BlocklistAccount, target: Pubkey) {
    if let Some(pos) = blk.blocked.iter().position(|&pk| pk == target) {
        blk.blocked.remove(pos);
        blk.unblock_count = blk.unblock_count.saturating_add(1);
    }
}

#[derive(Accounts)]
pub struct ModifyBlocklist<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub blocklist_account: Account<'info, BlocklistAccount>,
    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct BlocklistAccount {
    /// 本来このアカウントを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在ブロック中のユーザー Pubkey リスト
    pub blocked: Vec<Pubkey>,
    /// ブロック操作の累計回数
    pub block_count: u64,
    /// アンブロック操作の累計回数
    pub unblock_count: u64,
}
