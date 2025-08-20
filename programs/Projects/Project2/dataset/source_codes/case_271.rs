use anchor_lang::prelude::*;

declare_id!("UpgradeComp10010010010010010010010010010010");

#[program]
pub mod upgrade_components {
    use super::*;

    pub fn insert_component(ctx: Context<ModifyComp>, idx: u8, comp_id: u64) -> Result<()> {
        let uc = &mut ctx.accounts.upgrade;
        if (idx as usize) < uc.components.len() && uc.components[idx as usize].is_none() {
            uc.components[idx as usize] = Some(comp_id);
            uc.insert_count = uc.insert_count.saturating_add(1);
        } else {
            uc.replace_count = uc.replace_count.saturating_add(1);
        }
        Ok(())
    }

    pub fn clear_components(ctx: Context<ModifyComp>) -> Result<()> {
        let uc = &mut ctx.accounts.upgrade;
        for slot in uc.components.iter_mut() {
            *slot = None;
        }
        uc.insert_count = 0;
        uc.replace_count = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyComp<'info> {
    #[account(mut)]
    pub upgrade: Account<'info, UpgradeData>,
}

#[account]
pub struct UpgradeData {
    pub components: [Option<u64>; 5],
    pub insert_count: u64,
    pub replace_count: u64,
}
