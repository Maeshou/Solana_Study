use anchor_lang::prelude::*;

declare_id!("VulnInit0000000000000000000000000000000000");

#[program]
pub mod vuln_init {
    pub fn update_setting(ctx: Context<Upd>, val: u64) -> Result<()> {
        // settings.owner 未検証で誰でも書き換え可能
        ctx.accounts.settings.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Upd<'info> {
    #[account(init_if_needed, payer = user, space = 8 + 8)]
    pub settings: Account<'info, Settings>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Settings {
    pub owner: Pubkey,
    pub value: u64,
}
