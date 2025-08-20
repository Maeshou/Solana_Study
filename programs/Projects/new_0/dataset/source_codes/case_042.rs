use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA32mvTWf");

#[program]
pub mod fusion_tracker_003 {
    use super::*;

    pub fn fuse_items(ctx: Context<Ctx003>, item_id_1: u64, item_id_2: u64) -> Result<()> {
        let player = &mut ctx.accounts.player_data;

        let fused_result = item_id_1 ^ item_id_2; // 簡易な合成結果
        player.last_fused_id = fused_result;
        player.fusion_count += 1;
        player.total_cost += 10;

        Ok(())
    }

    pub fn show_fusion_log(ctx: Context<Ctx003>) -> Result<()> {
        let p = &ctx.accounts.player_data;
        msg!("Last Fused Item ID: {}", p.last_fused_id);
        msg!("Fusion Count: {}", p.fusion_count);
        msg!("Total Fusion Cost: {}", p.total_cost);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = player)]
    pub player_data: Account<'info, PlayerData>,
    #[account(signer)]
    pub player: Signer<'info>,
}

#[account]
pub struct PlayerData {
    pub player: Pubkey,
    pub last_fused_id: u64,
    pub fusion_count: u64,
    pub total_cost: u64,
}
