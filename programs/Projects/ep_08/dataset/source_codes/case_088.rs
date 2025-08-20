use anchor_lang::prelude::*;

declare_id!("DIV088088088088088088088088088088");

static mut CALL_COUNT: u32 = 0;

#[program]
pub mod case_088 {
    use super::*;

    pub fn bump_counter(ctx: Context<Counter088>, bump: u8) -> Result<()> {
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
pub struct Counter088<'info> {
    #[account(init, payer = signer, seeds = [b"delta088", bump.to_le_bytes().as_ref()], bump)]
    pub counter: Account<'info, CounterData088>,
    #[account(mut)] pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CounterData088 {
    pub count: u32,
    pub last_bump: u8,
}
