use anchor_lang::prelude::*;

declare_id!("TagStore42424242424242424242424242424242");

#[program]
pub mod tag_store42 {
    use super::*;

    /// タグを追加
    pub fn add_tag(ctx: Context<ModifyTag>, tag: String) -> Result<()> {
        let ts = &mut ctx.accounts.tags;
        ts.list.push(tag);
        Ok(())
    }

    /// 指定タグを削除
    pub fn remove_tag(ctx: Context<ModifyTag>, tag: String) -> Result<()> {
        let ts = &mut ctx.accounts.tags;
        ts.list.retain(|t| t != &tag);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyTag<'info> {
    #[account(mut)]
    pub tags: Account<'info, TagList>,
}

#[account]
pub struct TagList {
    pub list: Vec<String>,
}
