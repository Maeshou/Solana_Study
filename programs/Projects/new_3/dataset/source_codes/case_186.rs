use anchor_lang::prelude::*;
declare_id!("BlogVuln1111111111111111111111111111111");

/// ブログ投稿情報
#[account]
pub struct Post {
    pub author:    Pubkey,      // 投稿者
    pub title:     String,      // タイトル
    pub comments:  Vec<Pubkey>, // コメント投稿者一覧
}

/// コメント情報
#[account]
pub struct Comment {
    pub commenter: Pubkey,      // コメントを投稿したユーザー
    pub post:      Pubkey,      // 本来は Post.key() と一致すべき
    pub content:   String,      // コメント本文
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = author, space = 8 + 32 + 4 + 128 + 4 + (32 * 50))]
    pub post:       Account<'info, Post>,
    #[account(mut)]
    pub author:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddComment<'info> {
    /// Post.author == author.key() は不要だが例示
    #[account(mut, has_one = author)]
    pub post:       Account<'info, Post>,

    /// Comment.post ⇔ post.key() の検証がないため、
    /// 任意の Comment アカウントで処理を通過できる
    #[account(init, payer = commenter, space = 8 + 32 + 32 + 4 + 256)]
    pub comment:    Account<'info, Comment>,

    #[account(mut)]
    pub author:     Signer<'info>,
    #[account(mut)]
    pub commenter:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveComment<'info> {
    /// Comment.commenter == commenter.key() は検証される
    #[account(mut, has_one = commenter)]
    pub comment:    Account<'info, Comment>,

    /// Post.key() ⇔ comment.post の検証がないため、
    /// 任意の Comment で他の投稿のコメントを削除できる
    #[account(mut)]
    pub post:       Account<'info, Post>,

    pub commenter:  Signer<'info>,
}

#[program]
pub mod blog_vuln {
    use super::*;

    /// 投稿を作成
    pub fn create_post(ctx: Context<CreatePost>, title: String) -> Result<()> {
        let p = &mut ctx.accounts.post;
        p.author   = ctx.accounts.author.key();
        p.title    = title;
        // comments は init 時点で空 Vec
        Ok(())
    }

    /// コメントを追加
    pub fn add_comment(ctx: Context<AddComment>, content: String) -> Result<()> {
        let p = &mut ctx.accounts.post;
        let c = &mut ctx.accounts.comment;

        // 脆弱性ポイント:
        // c.post = p.key() と設定しているのみで、
        // Comment.post と Post.key() の一致検証を行っていない
        c.commenter = ctx.accounts.commenter.key();
        c.post      = p.key();
        c.content   = content;
        // コメント投稿者一覧に追加
        p.comments.push(c.commenter);
        Ok(())
    }

    /// コメントを削除（投稿者リストから取り除く）
    pub fn remove_comment(ctx: Context<RemoveComment>) -> Result<()> {
        let p = &mut ctx.accounts.post;
        let c = &ctx.accounts.comment;

        // 本来は必須：
        // require_keys_eq!(c.post, p.key(), ErrorCode::PostMismatch);

        // コメント投稿者一覧から該当ユーザーを除外
        p.comments.retain(|&pk| pk != c.commenter);
        Ok(())
    }
}
