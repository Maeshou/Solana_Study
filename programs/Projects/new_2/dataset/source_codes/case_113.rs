use anchor_lang::prelude::*;

declare_id!("WLstVuln444444444444444444444444444444444");

#[program]
pub mod whitelist_vuln {
    pub fn add_address(ctx: Context<ModifyWL>, addr: Pubkey) -> Result<()> {
        // wl.owner 検証なし
        let wl = &mut ctx.accounts.wl;
        wl.entries.push(addr);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyWL<'info> {
    #[account(mut)]
    pub wl: Account<'info, WhiteList>,
}

#[account]
pub struct WhiteList {
    pub owner: Pubkey,
    pub entries: Vec<Pubkey>,
}
