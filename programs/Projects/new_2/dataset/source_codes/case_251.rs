use anchor_lang::prelude::*;

declare_id!("VulnEx58000000000000000000000000000000000058");

#[program]
pub mod refund {
    pub fn request(ctx: Context<Ctx8>, amount: u64) -> Result<()> {
        // log_buf: OWNER CHECK SKIPPED
        ctx.accounts.log_buf.data.borrow_mut().extend_from_slice(&amount.to_le_bytes());

        // refund_acc: has_one = user
        let rf = &mut ctx.accounts.refund_acc;
        rf.pending = rf.pending.saturating_add(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx8<'info> {
    #[account(mut)]
    pub log_buf: AccountInfo<'info>,

    #[account(mut, has_one = user)]
    pub refund_acc: Account<'info, RefundAcc>,
    pub user: Signer<'info>,
}

#[account]
pub struct RefundAcc {
    pub user: Pubkey,
    pub pending: u64,
}
