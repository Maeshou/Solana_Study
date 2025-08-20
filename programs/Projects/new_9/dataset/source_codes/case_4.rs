use anchor_lang::prelude::*;

declare_id!("NFTQuestClose4444444444444444444444444444");

#[program]
pub mod quest_pass_archiver {
    use super::*;

    pub fn archive_pass(ctx: Context<ArchivePass>, seed: u64) -> Result<()> {
        let pass_ai = ctx.accounts.pass.to_account_info();
        let sink_ai = ctx.accounts.sink.to_account_info();

        let start = pass_ai.lamports();
        let mut acc = seed ^ (start.rotate_left(9));
        (0..5).for_each(|k| {
            let r = ((k as u64 + 19) * 41).rotate_left((k * 3) as u32);
            acc = acc.wrapping_add(r ^ (k as u64).wrapping_mul(73)).rotate_right(2);
        });

        let send = start;
        **sink_ai.lamports.borrow_mut() = sink_ai.lamports().checked_add(send).unwrap();
        let mut w = pass_ai.lamports.borrow_mut();
        let prev = *w;
        *w = prev.checked_sub(send).unwrap();

        ctx.accounts.pass.meta = acc ^ (seed * 777);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ArchivePass<'info> {
    #[account(mut)]
    pub pass: Account<'info, PassMeta>,
    /// CHECK:
    #[account(mut)]
    pub sink: UncheckedAccount<'info>,
}
#[account]
pub struct PassMeta { pub meta: u64 }
