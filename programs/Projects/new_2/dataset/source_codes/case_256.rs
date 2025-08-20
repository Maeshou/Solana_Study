use anchor_lang::prelude::*;

declare_id!("VulnEx69000000000000000000000000000000000069");

#[program]
pub mod example69 {
    pub fn bump_counter(ctx: Context<Ctx69>) -> Result<()> {
        // info_acc is unchecked
        ctx.accounts.info_acc.data.borrow_mut()[0] = ctx.accounts.info_acc.data.borrow()[0].saturating_add(1);
        // counter_acc is has_one = setter
        ctx.accounts.counter_acc.count = ctx.accounts.counter_acc.count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx69<'info> {
    #[account(mut)]
    pub info_acc: AccountInfo<'info>,
    #[account(mut, has_one = setter)]
    pub counter_acc: Account<'info, CounterAcc>,
    pub setter: Signer<'info>,
}

#[account]
pub struct CounterAcc {
    pub setter: Pubkey,
    pub count: u64,
}
