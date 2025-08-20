use anchor_lang::prelude::*;
use std::collections::BTreeSet;

declare_id!("MarketBan6666666666666666666666666666666666");

#[program]
pub mod marketplace_ban {
    use super::*;

    pub fn ban_user(ctx: Context<ModifyBan>, target: Pubkey) -> Result<()> {
        ctx.accounts.list.banned.insert(target);
        Ok(())
    }

    pub fn unban_user(ctx: Context<ModifyBan>, target: Pubkey) -> Result<()> {
        ctx.accounts.list.banned.remove(&target);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyBan<'info> {
    #[account(mut)]
    pub list: Account<'info, BanListData>,
    pub admin: Signer<'info>,
}

#[account]
pub struct BanListData {
    pub banned: BTreeSet<Pubkey>,
}
