use anchor_lang::prelude::*;

declare_id!("OwnChkB7000000000000000000000000000000007");

#[program]
pub mod post_editor {
    pub fn edit_post(ctx: Context<Edit>, new_body: String) -> Result<()> {
        let p = &mut ctx.accounts.post;
        // has_one で author チェック済み
        p.body         = new_body;
        p.edit_count   = p.edit_count.saturating_add(1);
        p.last_edited  = Clock::get()?.unix_timestamp;

        // temp_cache は unchecked
        ctx.accounts.temp_cache.data.borrow_mut().fill(0);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Edit<'info> {
    #[account(mut, has_one = author)]
    pub post: Account<'info, PostData>,
    pub author: Signer<'info>,
    /// CHECK: 一時キャッシュ、所有者検証なし
    #[account(mut)]
    pub temp_cache: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct PostData {
    pub author: Pubkey,
    pub body: String,
    pub edit_count: u64,
    pub last_edited: i64,
}
