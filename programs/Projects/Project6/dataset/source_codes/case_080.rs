use anchor_lang::prelude::*;

declare_id!("GACHAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod gacha_system_program {
    use super::*;
    /// 通貨を消費し、ランダムなアイテムIDのリストを生成します。
    pub fn draw_from_gacha(ctx: Context<DrawGacha>, amount: u32) -> Result<()> {
        let player_resources = &mut ctx.accounts.player_resources;
        let gacha_receipt = &mut ctx.accounts.gacha_receipt;
        let clock = Clock::get()?;
        
        let cost_per_draw = 100;
        player_resources.gems = player_resources.gems.saturating_sub(cost_per_draw * amount);

        let mut received_items = Vec::new();
        let loot_table: [u32; 5] = [1001, 2005, 3010, 4050, 9999];

        for i in 1..=amount {
            let seed = clock.slot.saturating_add(i as u64);
            let random_index = seed % (loot_table.len() as u64);
            received_items.push(loot_table[random_index as usize]);
        }
        
        gacha_receipt.player = *ctx.accounts.player.key;
        gacha_receipt.items_won = received_items;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DrawGacha<'info> {
    #[account(mut, has_one = owner, constraint = player_resources.gems >= 100 @ GameErrorCode::NotEnoughGems)]
    pub player_resources: Account<'info, PlayerResources>,
    #[account(init, payer = player, space = 8 + 32 + 4 + (10 * 4))]
    pub gacha_receipt: Account<'info, GachaReceipt>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlayerResources {
    pub owner: Pubkey,
    pub gems: u32,
}

#[account]
pub struct GachaReceipt {
    pub player: Pubkey,
    pub items_won: Vec<u32>,
}

#[error_code]
pub enum GameErrorCode {
    #[msg("Not enough gems to draw from gacha.")]
    NotEnoughGems,
}