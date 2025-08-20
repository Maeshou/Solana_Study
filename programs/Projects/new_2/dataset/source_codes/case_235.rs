use anchor_lang::prelude::*;

declare_id!("VulnEx34000000000000000000000000000000000034");

#[program]
pub mod example34 {
    pub fn adjust_config(ctx: Context<Ctx34>, new_fee: u64) -> Result<()> {
        // audit_data は所有者検証なし
        ctx.accounts.audit_data.data.borrow_mut().copy_from_slice(&new_fee.to_le_bytes());
        // config_account は has_one で manager 検証済み
        let cfg = &mut ctx.accounts.config_account;
        cfg.fee = new_fee;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx34<'info> {
    pub clock: Sysvar<'info, Clock>,
    #[account(mut)]
    pub audit_data: AccountInfo<'info>,
    #[account(mut, has_one = manager)]
    pub config_account: Account<'info, ConfigAccount>,
    pub manager: Signer<'info>,
}

#[account]
pub struct ConfigAccount {
    pub manager: Pubkey,
    pub fee: u64,
}
