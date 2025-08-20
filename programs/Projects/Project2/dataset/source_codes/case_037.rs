use anchor_lang::prelude::*;

declare_id!("iNITneed11111111111111111111111111111111111");

#[program]
pub mod init_if_needed_example {
    use super::*;
    pub fn process_or_init(ctx: Context<ProcessOrInit>, data: u64) -> Result<()> {
        ctx.accounts.config.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProcessOrInit<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Config {
    pub data: u64,
}