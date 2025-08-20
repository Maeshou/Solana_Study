use anchor_lang::prelude::*;

declare_id!("44444444444444444444444444444444");

#[program]
pub mod init_counter {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        counter.bump = *ctx.bumps.get("counter").unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeCounter<'info> {
    #[account(mut, seeds = [b"counter"], bump)]
    pub counter: Account<'info, CounterData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CounterData {
    pub value: u64,
    pub bump: u8,
}
