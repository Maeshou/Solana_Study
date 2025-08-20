use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgForumSvc01");

#[program]
pub mod forum_service {
    use super::*;

    /// 投稿の内容を編集するが、
    /// post_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn edit_post(ctx: Context<ModifyPost>, new_content: String) -> Result<()> {
        let post = &mut ctx.accounts.post_account;
        apply_edit(post, new_content);
        Ok(())
    }

    /// 投稿を論理削除するが、
    /// post_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn delete_post(ctx: Context<ModifyPost>) -> Result<()> {
        let post = &mut ctx.accounts.post_account;
        apply_delete(post);
        Ok(())
    }
}

/// 投稿内容を更新し、編集履歴をインクリメントするヘルパー関数
fn apply_edit(post: &mut PostAccount, content: String) {
    post.content = content;
    post.edit_count = post.edit_count.checked_add(1).unwrap();
}

/// 論理削除フラグを立て、削除履歴をインクリメントするヘルパー関数
fn apply_delete(post: &mut PostAccount) {
    post.deleted = true;
    post.delete_count = post.delete_count.checked_add(1).unwrap();
}

#[derive(Accounts)]
pub struct ModifyPost<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub post_account: Account<'info, PostAccount>,
    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct PostAccount {
    /// 本来この投稿を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 投稿本文
    pub content: String,
    /// 編集回数
    pub edit_count: u64,
    /// 論理削除フラグ
    pub deleted: bool,
    /// 削除操作回数
    pub delete_count: u64,
}
