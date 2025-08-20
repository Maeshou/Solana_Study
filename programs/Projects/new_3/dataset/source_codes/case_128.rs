use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBlockRematch02");

#[program]
pub mod blocklist_service {
    use super::*;

    /// ユーザーをブロックリストに追加するが、
    /// has_one = owner のみ検証しており、
    /// 実際の操作ユーザー（user）の照合がないため、
    /// 攻撃者が他人のアカウントでブロック追加を行える
    pub fn add_to_blocklist(ctx: Context<ModifyBlocklist>, target: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.blocklist_account;
        // 1. 最終にブロックしたユーザーを記録
        acct.last_blocked = target;
        // 2. ブロック操作回数をインクリメント
        acct.block_count = acct.block_count + 1;
        Ok(())
    }

    /// ユーザーをブロックリストから削除するが、
    /// has_one = owner のみ検証しており、
    /// 実際の操作ユーザー（user）の照合がないため、
    /// 攻撃者が他人のアカウントでブロック解除を行える
    pub fn remove_from_blocklist(ctx: Context<ModifyBlocklist>, target: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.blocklist_account;
        // 1. 最終に解除したユーザーを記録
        acct.last_unblocked = target;
        // 2. 解除操作回数をインクリメント
        acct.unblock_count = acct.unblock_count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyBlocklist<'info> {
    #[account(mut, has_one = owner)]
    /// 本来は has_one = user を追加して
    /// blocklist_account.user と ctx.accounts.user.key() の照合を行うべき
    pub blocklist_account: Account<'info, BlocklistAccount>,
    /// アカウント所有者のみ検証
    pub owner: Signer<'info>,
    /// 実際の操作ユーザー（検証漏れ）
    pub user: Signer<'info>,
}

#[account]
pub struct BlocklistAccount {
    /// 本来このブロックリストを管理するべきユーザー
    pub owner: Pubkey,
    /// 最後にブロックされたユーザー
    pub last_blocked: Pubkey,
    /// 累計ブロック操作回数
    pub block_count: u64,
    /// 最後に解除されたユーザー
    pub last_unblocked: Pubkey,
    /// 累計解除操作回数
    pub unblock_count: u64,
}
