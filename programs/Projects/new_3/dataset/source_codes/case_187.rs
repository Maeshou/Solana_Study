use anchor_lang::prelude::*;
declare_id!("ConModPopVuln11111111111111111111111111");

/// コンテンツ情報
#[account]
pub struct Content {
    pub author:        Pubkey,        // 投稿者
    pub text:          String,        // 本文
    pub flagged_users: Vec<Pubkey>,   // フラグを立てたユーザー一覧
}

/// フラグ記録
#[account]
pub struct FlagRecord {
    pub user:     Pubkey,             // フラグを立てたユーザー
    pub content:  Pubkey,             // 本来は Content.key() と一致すべき
    pub reason:   String,             // フラグ理由
}

#[derive(Accounts)]
pub struct CreateContent<'info> {
    #[account(init, payer = author, space = 8 + 32 + 4 + 280 + 4 + (32 * 20))]
    pub content:  Account<'info, Content>,
    #[account(mut)]
    pub author:   Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FlagContent<'info> {
    /// Content.author == author.key() は検証される
    #[account(mut, has_one = author)]
    pub content:  Account<'info, Content>,

    /// FlagRecord.content ⇔ content.key() の検証がないため、
    /// 偽物のレコードで任意のコンテンツにフラグを立てられる
    #[account(init, payer = user, space = 8 + 32 + 32 + 4 + 128)]
    pub record:   Account<'info, FlagRecord>,

    #[account(mut)]
    pub author:   Signer<'info>,
    #[account(mut)]
    pub user:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveFlag<'info> {
    /// FlagRecord.user == moderator.key() は検証される
    #[account(mut, has_one = user)]
    pub record:   Account<'info, FlagRecord>,

    /// Content.key() ⇔ record.content の検証がないため、
    /// 偽物のレコードで別のコンテンツのフラグを解除できる
    #[account(mut)]
    pub content:  Account<'info, Content>,

    pub user:     Signer<'info>,
}

#[program]
pub mod content_moderation_pop_vuln {
    use super::*;

    pub fn create_content(ctx: Context<CreateContent>, text: String) -> Result<()> {
        let ct = &mut ctx.accounts.content;
        ct.author = ctx.accounts.author.key();
        ct.text   = text;
        // flagged_users は init 時に空 Vec
        Ok(())
    }

    pub fn flag_content(ctx: Context<FlagContent>, reason: String) -> Result<()> {
        let ct = &mut ctx.accounts.content;
        let fr = &mut ctx.accounts.record;

        // 脆弱性ポイント:
        // fr.content = ct.key(); の一致検証がない
        fr.user    = ctx.accounts.user.key();
        fr.content = ct.key();
        fr.reason  = reason;

        // Vec::push でフラグユーザー一覧に追加
        ct.flagged_users.push(fr.user);
        Ok(())
    }

    pub fn resolve_flag(ctx: Context<ResolveFlag>) -> Result<()> {
        let ct = &mut ctx.accounts.content;

        // 本来必要:
        // require_keys_eq!(ctx.accounts.record.content, ct.key(), ErrorCode::Mismatch);

        // Vec::pop で最後に追加されたユーザーを除去（単一操作・分岐なし）
        ct.flagged_users.pop();

        Ok(())
    }
}
