use anchor_lang::prelude::*;

declare_id!("OwnChkC4000000000000000000000000000000004");

#[program]
pub mod equipment {
    pub fn upgrade(
        ctx: Context<Upgrade>,
        slot_id: u8,
    ) -> Result<()> {
        let eq = &mut ctx.accounts.equipment;
        // 属性検証で eq.owner をチェック
        if (slot_id as usize) < eq.levels.len() {
            eq.levels[slot_id as usize] = eq.levels[slot_id as usize].saturating_add(1);
            eq.upgrade_count = eq.upgrade_count.saturating_add(1);
        }

        // log_buf は unchecked
        ctx.accounts.log_buf.data.borrow_mut().extend_from_slice(&[slot_id]);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Upgrade<'info> {
    #[account(mut, has_one = owner)]
    pub equipment: Account<'info, EquipmentData>,
    pub owner: Signer<'info>,
    /// CHECK: ログバッファ、所有者検証なし
    #[account(mut)]
    pub log_buf: AccountInfo<'info>,
}

#[account]
pub struct EquipmentData {
    pub owner: Pubkey,
    pub levels: [u8; 5],
    pub upgrade_count: u64,
}
