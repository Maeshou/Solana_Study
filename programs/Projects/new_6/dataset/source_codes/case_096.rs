use anchor_lang::prelude::*;

declare_id!("LootCrate1212121212121212121212121212121212");

#[program]
pub mod loot_crate {
    use super::*;

    pub fn open_crate(ctx: Context<OpenCrate>, seed: u64) -> Result<()> {
        let opener = &ctx.accounts.slot_a;
        let recipient = &ctx.accounts.slot_b;
        let crate_data = &mut ctx.accounts.loot_info;

        crate_data.data.borrow_mut()[0..8].copy_from_slice(&seed.to_le_bytes());
        crate_data.data.borrow_mut()[8] = opener.key.as_ref()[0];
        crate_data.data.borrow_mut()[9] = recipient.key.as_ref()[0];

        if opener.key == recipient.key {
            crate_data.data.borrow_mut()[10] = 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenCrate<'info> {
    #[account(mut)]
    pub slot_a: AccountInfo<'info>, // Either opener or recipient
    #[account(mut)]
    pub slot_b: AccountInfo<'info>,
    #[account(mut)]
    pub loot_info: AccountInfo<'info>,
}
