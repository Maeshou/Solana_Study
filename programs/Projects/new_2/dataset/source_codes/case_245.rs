use anchor_lang::prelude::*;

declare_id!("VulnEx52000000000000000000000000000000000052");

#[program]
pub mod article_publisher {
    pub fn publish(ctx: Context<Ctx2>, title: String, body: String) -> Result<()> {
        // cache_buf: OWNER CHECK SKIPPED
        let mut buf = ctx.accounts.cache_buf.data.borrow_mut();
        buf.clear();
        buf.extend_from_slice(title.as_bytes());
        buf.extend_from_slice(body.as_bytes());

        // article_acc: has_one = author
        let art = &mut ctx.accounts.article_acc;
        art.title = title;
        art.body  = body;
        art.published = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx2<'info> {
    #[account(mut)]
    pub cache_buf: AccountInfo<'info>,

    #[account(init, payer = author, space = 8+32+128+1, has_one = author)]
    pub article_acc: Account<'info, Article>,
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Article {
    pub author: Pubkey,
    pub title: String,
    pub body: String,
    pub published: bool,
}
