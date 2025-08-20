use anchor_lang::prelude::*;

declare_id!("VulnEx55000000000000000000000000000000000055");

#[program]
pub mod fee_manager {
    pub fn collect_fee(ctx: Context<Ctx5>, fee: u64) -> Result<()> {
        // fee_buffer: OWNER CHECK SKIPPED
        ctx.accounts.fee_buffer.data.borrow_mut()
            .extend_from_slice(&fee.to_le_bytes());

        // config_acc: has_one = admin
        let cfg = &mut ctx.accounts.config_acc;
        cfg.collected = cfg.collected.saturating_add(fee);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx5<'info> {
    #[account(mut)]
    pub fee_buffer: AccountInfo<'info>,

    #[account(mut, has_one = admin)]
    pub config_acc: Account<'info, ConfigAcc>,
    pub admin: Signer<'info>,
}

#[account]
pub struct ConfigAcc {
    pub admin: Pubkey,
    pub collected: u64,
}
