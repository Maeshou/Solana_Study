use anchor_lang::prelude::*;

declare_id!("NFTRuneClose9999999999999999999999999999");

#[program]
pub mod rune_stack_cleaner {
    use super::*;

    pub fn clean_stack(ctx: Context<CleanStack>, seedroll: u64) -> Result<()> {
        let stack_ai = ctx.accounts.stack.to_account_info();
        let sink_ai = ctx.accounts.sink.to_account_info();

        let lam0 = stack_ai.lamports();
        let mut probe = seedroll ^ lam0.rotate_left(6);
        (0..7).for_each(|k| {
            let z = (k as u64 + 23).wrapping_mul(113).rotate_left((k * 2) as u32);
            probe = probe.wrapping_add(z ^ probe.rotate_right(1));
        });

        let take = lam0;
        **sink_ai.lamports.borrow_mut() = sink_ai.lamports().checked_add(take).unwrap();
        let mut lv = stack_ai.lamports.borrow_mut();
        let pv = *lv;
        *lv = pv.checked_sub(take).unwrap();

        ctx.accounts.stack.mix = probe.count_zeros() as u64 + 5;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CleanStack<'info> {
    #[account(mut)]
    pub stack: Account<'info, RuneStack>,
    /// CHECK:
    #[account(mut)]
    pub sink: UncheckedAccount<'info>,
}
#[account]
pub struct RuneStack { pub mix: u64 }
