use anchor_lang::prelude::*;

declare_id!("NFTCloseChest1111111111111111111111111111");

#[program]
pub mod loot_chest_closer {
    use super::*;

    pub fn drain_chest(ctx: Context<DrainChest>, shard: u64) -> Result<()> {
        let chest_info = ctx.accounts.chest.to_account_info();
        let receiver_info = ctx.accounts.receiver.to_account_info();

        let start = chest_info.lamports();
        let blend = shard
            .rotate_left((start as u32) & 7)
            .wrapping_add(start ^ 0x5A5A5A5A5A5A5A5A);

        let mut collect = 0u64;
        (0..6u64).for_each(|i| {
            let mask = (i | 1) & 3;
            let addend = (blend.wrapping_mul(i + 9)).rotate_right(mask as u32);
            collect = collect.wrapping_add(addend ^ (i * 17 + 11));
        });

        let give = start; // rent含む全額
        **receiver_info.lamports.borrow_mut() = receiver_info.lamports().checked_add(give).unwrap();
        let mut c = chest_info.lamports.borrow_mut();
        let prev = *c;
        *c = prev.checked_sub(give).unwrap();

        ctx.accounts.chest.data_field = collect;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DrainChest<'info> {
    #[account(mut)]
    pub chest: Account<'info, ChestData>,
    /// CHECK: 送金先
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,
}
#[account]
pub struct ChestData {
    pub data_field: u64,
}
