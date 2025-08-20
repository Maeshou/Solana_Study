use anchor_lang::prelude::*;

declare_id!("DIV038038038038038038038038038038");

static mut CALL_COUNT: u32 = 0;

#[program]
pub mod case_038 {
    use super::*;

    pub fn bump_counter(ctx: Context<Counter038>, bump: u8) -> Result<()> {
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
pub struct Counter038<'info> {
    #[account(init, payer = signer, seeds = [b"delta038", bump.to_le_bytes().as_ref()], bump)]
    pub counter: Account<'info, CounterData038>,
    #[account(mut)] pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CounterData038 {
    pub count: u32,
    pub last_bump: u8,
}
