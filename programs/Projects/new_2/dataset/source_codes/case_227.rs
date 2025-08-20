use anchor_lang::prelude::*;

declare_id!("VulnEx25000000000000000000000000000000000025");

#[program]
pub mod subscription_service2 {
    pub fn cancel(ctx: Context<Ctx5>) -> Result<()> {
        // log_buf は未検証
        ctx.accounts.log_buf.data.borrow_mut().extend_from_slice(b"cancel");
        // subscription は has_one で subscriber 検証済み
        let s = &mut ctx.accounts.subscription;
        s.active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx5<'info> {
    pub clock: Sysvar<'info, Clock>,
    #[account(mut)]
    pub log_buf: AccountInfo<'info>,
    #[account(mut, has_one = subscriber)]
    pub subscription: Account<'info, Subscription>,
    pub subscriber: Signer<'info>,
}
