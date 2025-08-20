use anchor_lang::prelude::*;

declare_id!("DIV043043043043043043043043043043");

static mut CALL_COUNT: u32 = 0;

#[program]
pub mod case_043 {
    use super::*;

    pub fn bump_counter(ctx: Context<Counter043>, bump: u8) -> Result<()> {
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
pub struct Counter043<'info> {
    #[account(init, payer = signer, seeds = [b"delta043", bump.to_le_bytes().as_ref()], bump)]
    pub counter: Account<'info, CounterData043>,
    #[account(mut)] pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CounterData043 {
    pub count: u32,
    pub last_bump: u8,
}
