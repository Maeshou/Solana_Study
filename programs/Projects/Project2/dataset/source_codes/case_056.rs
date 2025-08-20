use anchor_lang::prelude::*;

declare_id!("Blog111111111111111111111111111111111111");

#[program]
pub mod blog_platform {
    use super::*;
    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> Result<()> {
        let post = &mut ctx.accounts.blog_post;
        post.authority = *ctx.accounts.authority.key;
        post.title = title;
        post.content = content;
        Ok(())
    }

    pub fn delete_post(ctx: Context<DeletePost>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, content: String)]
pub struct CreatePost<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + title.len() + 4 + content.len()
    )]
    pub blog_post: Account<'info, BlogPost>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeletePost<'info> {
    // 所有者チェックと権限チェックを同時に行う
    #[account(
        mut,
        close = authority,
        has_one = authority
    )]
    pub blog_post: Account<'info, BlogPost>,
    pub authority: Signer<'info>,
}

#[account]
pub struct BlogPost {
    pub authority: Pubkey,
    pub title: String,
    pub content: String,
}