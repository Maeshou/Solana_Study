use anchor_lang::prelude::*;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpH1I2J3K4L5M6N7O8P9Q0R1S2T3");

#[program]
pub mod tag_manager {
    use super::*;

    /// タグアカウントを初期化
    pub fn initialize(
        ctx: Context<InitializeTagManager>,
        bump: u8,
    ) -> ProgramResult {
        let account = &mut ctx.accounts.tag_account;
        account.owner = *ctx.accounts.user.key;
        account.bump = bump;
        account.tags = Default::default();
        Ok(())
    }

    /// タグを追加
    pub fn add_tag(
        ctx: Context<AddTag>,
        tag: String,
    ) -> ProgramResult {
        let account = &mut ctx.accounts.tag_account;
        account.tags.push(tag);
        Ok(())
    }

    /// すべてのタグをクリア
    pub fn clear_tags(
        ctx: Context<ClearTags>,
    ) -> ProgramResult {
        let account = &mut ctx.accounts.tag_account;
        account.tags.clear();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeTagManager<'info> {
    #[account(
        init,
        seeds = [b"tag", user.key().as_ref()],
        bump = bump,
        payer = user,
        space = 8 + 32 + 1 + 4 + 32 * 20,
    )]
    pub tag_account: Account<'info, TagAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct AddTag<'info> {
    #[account(
        mut,
        seeds = [b"tag", tag_account.owner.as_ref()],
        bump = tag_account.bump,
        has_one = owner,
    )]
    pub tag_account: Account<'info, TagAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearTags<'info> {
    #[account(
        mut,
        seeds = [b"tag", tag_account.owner.as_ref()],
        bump = tag_account.bump,
        has_one = owner,
    )]
    pub tag_account: Account<'info, TagAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct TagAccount {
    pub owner: Pubkey,
    pub bump: u8,
    pub tags: Vec<String>,
}
