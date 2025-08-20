use anchor_lang::prelude::*;

declare_id!("NFTCraft666666666666666666666666666666666666");

#[program]
pub mod nft_craft_station {
    use super::*;

    pub fn forge_item(ctx: Context<ForgeItem>, seed: u64, affinity: u8) -> Result<()> {
        let workstation = &mut ctx.accounts.workshop;
        let crafter = &ctx.accounts.entity;
        let output_log = &mut ctx.accounts.creation_log;

        let infused_value = seed.rotate_left(affinity as u32);
        output_log.data.borrow_mut()[0..8].copy_from_slice(&infused_value.to_le_bytes());

        if affinity > 200 {
            workstation.high_affinity_forges += 1;
            for i in 0..4 {
                output_log.data.borrow_mut()[8 + i] = i as u8 * affinity;
            }
        }

        if seed % 17 == 0 {
            output_log.data.borrow_mut()[12] = 0xEE;
            workstation.special_seed_usage += 1;
        }

        if workstation.current_state == 0 {
            workstation.current_state = 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ForgeItem<'info> {
    #[account(mut)]
    pub workshop: AccountInfo<'info>, // Not properly typed
    #[account(mut)]
    pub entity: AccountInfo<'info>,   // Could be crafter or validator
    #[account(mut)]
    pub creation_log: AccountInfo<'info>,
}
