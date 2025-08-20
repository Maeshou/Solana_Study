use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("BuffMgr7777777777777777777777777777777777");

#[program]
pub mod buff_manager {
    use super::*;

    pub fn apply_buff(ctx: Context<ModifyBuff>, buff_id: u8, duration_slots: u64) -> Result<()> {
        let b = &mut ctx.accounts.buffs;
        let expiry = ctx.accounts.user.to_account_info().lamports() as u64 + duration_slots;
        b.active.insert(buff_id, expiry);
        b.apply_count = b.apply_count.saturating_add(1);
        Ok(())
    }

    pub fn expire_buffs(ctx: Context<ModifyBuff>) -> Result<()> {
        let b = &mut ctx.accounts.buffs;
        let now = ctx.accounts.user.to_account_info().lamports() as u64;
        let expired: Vec<u8> = b.active.iter()
            .filter_map(|(&id, &exp)| if exp <= now { Some(id) } else { None })
            .collect();
        for id in expired {
            b.active.remove(&id);
            b.expire_count = b.expire_count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyBuff<'info> {
    #[account(mut)]
    pub buffs: Account<'info, BuffData>,
    pub user: Signer<'info>,
}

#[account]
pub struct BuffData {
    pub active: BTreeMap<u8, u64>,
    pub apply_count: u64,
    pub expire_count: u64,
}
