use anchor_lang::prelude::*;

declare_id!("DIV028028028028028028028028028028");

static mut CALL_COUNT: u32 = 0;

#[program]
pub mod case_028 {
    use super::*;

    pub fn bump_counter(ctx: Context<Counter028>, bump: u8) -> Result<()> {
        unsafe {
            CALL_COUNT += 1;
        }
        ctx.accounts.counter.count = unsafe { CALL_COUNT };
        ctx.accounts.counter.last_bump = bump;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Counter028<'info> {
    #[account(init, payer = signer, seeds = [b"delta028", bump.to_le_bytes().as_ref()], bump)]
    pub counter: Account<'info, CounterData028>,
    #[account(mut)] pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CounterData028 {
    pub count: u32,
    pub last_bump: u8,
}
