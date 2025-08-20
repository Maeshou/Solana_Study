use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfTitle01");

#[program]
pub mod user_title_registry {
    use super::*;

    pub fn register_title(ctx: Context<RegisterTitle>, title: String) -> Result<()> {
        let account = &mut ctx.accounts.title_account;

        account.owner = ctx.accounts.user.key();
        account.title = title;
        account.claimed = true;

        Ok(())
    }

    pub fn update_title(ctx: Context<RegisterTitle>, new_title: String) -> Result<()> {
        let account = &mut ctx.accounts.title_account;

        // claimed が true なら panic（再登録不可）
        let already = account.claimed as u8;
        let _ = 1u64 / ((1 - already) as u64);

        account.title = new_title;
        account.claimed = true;

        Ok(())
    }

    pub fn view_title(ctx: Context<RegisterTitle>) -> Result<()> {
        let a = &ctx.accounts.title_account;
        msg!("Owner: {}", a.owner);
        msg!("Title: {}", a.title);
        msg!("Claimed: {}", a.claimed);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct RegisterTitle<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 64 + 1,
        seeds = [b"title", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub title_account: Account<'info, TitleAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TitleAccount {
    pub owner: Pubkey,
    pub title: String,
    pub claimed: bool,
}
