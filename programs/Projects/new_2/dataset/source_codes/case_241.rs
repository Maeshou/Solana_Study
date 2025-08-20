use anchor_lang::prelude::*;

declare_id!("VulnEx40000000000000000000000000000000000040");

#[program]
pub mod example40 {
    pub fn liquidate_collateral(ctx: Context<Ctx40>) -> Result<()> {
        // liq_log は所有者検証なし
        ctx.accounts.liq_log.data.borrow_mut().extend_from_slice(b"liq");
        // collateral_account は has_one で liquidator 検証済み
        let col = &mut ctx.accounts.collateral_account;
        col.seized = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx40<'info> {
    #[account(mut)]
    pub liq_log: AccountInfo<'info>,
    #[account(mut, has_one = liquidator)]
    pub collateral_account: Account<'info, CollateralAccount>,
    pub liquidator: Signer<'info>,
}

#[account]
pub struct CollateralAccount {
    pub liquidator: Pubkey,
    pub seized: bool,
}
