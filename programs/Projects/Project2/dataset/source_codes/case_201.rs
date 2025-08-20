use anchor_lang::prelude::*;

declare_id!("Inv01Game0000000000000000000000000000001");

#[program]
pub mod inventory_manager {
    use super::*;

    pub fn init_item(ctx: Context<InitItem>, nft_id: u64, rarity: u8) -> Result<()> {
        let item = &mut ctx.accounts.item;
        item.id = nft_id;
        item.rarity = rarity;
        item.experience = 0;
        item.level = 1;
        Ok(())
    }

    pub fn gain_experience(ctx: Context<ModifyItem>, amount: u64) -> Result<()> {
        let item = &mut ctx.accounts.item;
        item.experience = item.experience.checked_add(amount).unwrap();
        Ok(())
    }

    pub fn level_up(ctx: Context<ModifyItem>) -> Result<()> {
        let item = &mut ctx.accounts.item;
        if item.experience >= item.level * 100 {
            item.experience -= item.level * 100;
            item.level += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitItem<'info> {
    #[account(
        init,
        seeds = [b"item", user.key().as_ref(), &nft_id.to_le_bytes()],
        bump,
        payer = user,
        space = 8 + 8 + 1 + 8 + 8
    )]
    pub item: Account<'info, ItemData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyItem<'info> {
    #[account(
        mut,
        seeds = [b"item", user.key().as_ref(), &item.id.to_le_bytes()],
        bump
    )]
    pub item: Account<'info, ItemData>,
    pub user: Signer<'info>,
}

#[account]
pub struct ItemData {
    pub id: u64,
    pub rarity: u8,
    pub experience: u64,
    pub level: u64,
}
