use anchor_lang::prelude::*;

declare_id!("EqpSlot4444444444444444444444444444444444");

#[program]
pub mod equip_slots {
    use super::*;

    pub fn init_slots(ctx: Context<InitSlots>) -> Result<()> {
        // 固定配列は既に 0 で初期化
        Ok(())
    }

    pub fn equip(ctx: Context<ModifySlots>, slot_idx: u8, nft_id: u64) -> Result<()> {
        let s = &mut ctx.accounts.slots;
        if (slot_idx as usize) < s.slots.len() {
            s.slots[slot_idx as usize] = nft_id;
        }
        Ok(())
    }

    pub fn unequip(ctx: Context<ModifySlots>, slot_idx: u8) -> Result<()> {
        let s = &mut ctx.accounts.slots;
        if (slot_idx as usize) < s.slots.len() {
            s.slots[slot_idx as usize] = 0;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSlots<'info> {
    #[account(init, payer = user, space = 8 + 8 * 5)]
    pub slots: Account<'info, SlotData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifySlots<'info> {
    #[account(mut)] pub slots: Account<'info, SlotData>,
}

#[account]
pub struct SlotData {
    /// 0 は未装備
    pub slots: [u64; 5],
}
