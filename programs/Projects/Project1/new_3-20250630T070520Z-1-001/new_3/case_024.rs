use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgFriendSvc01");

#[program]
pub mod friend_service {
    use super::*;

    /// フレンドリストにユーザーを追加するが、
    /// friend_list.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn add_friend(ctx: Context<AddFriend>, new_friend: Pubkey) -> Result<()> {
        let list = &mut ctx.accounts.friend_list;

        // 1. フレンド数をインクリメント
        list.friend_count = list.friend_count.checked_add(1).unwrap();

        // 2. 最後に追加されたフレンドを記録
        list.last_added = new_friend;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddFriend<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付けて照合を行うべき
    pub friend_list: Account<'info, FriendList>,

    /// フレンド追加をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct FriendList {
    /// 本来このリストを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在のフレンド数
    pub friend_count: u64,
    /// 最後に追加されたフレンドの Pubkey
    pub last_added: Pubkey,
}
