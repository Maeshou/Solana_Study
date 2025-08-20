use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("CrAfTLoG0000000000000000000000000000003");

#[program]
pub mod craft_log {
    use super::*;

    pub fn record_craft(ctx: Context<RecordCraft>, recipe: [u8; 12], energy: u32, bump: u8) -> Result<()> {
        // レシピの整形と簡易コスト
        let mut rec = recipe;
        let mut cost: u32 = 0;
        for i in 0..rec.len() {
            let v = rec[i];
            if !(v.is_ascii_alphanumeric()) { rec[i] = b'*'; }
            cost = cost.wrapping_add((v as u32) * (i as u32 + 7));
        }
        let mut e = energy;
        if e > cost { e = cost; }

        // 入力 bump のまま PDA を派生（該当点）
        let seeds = [&ctx.accounts.crafter.key().to_bytes()[..], &rec[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(CraftErr::Cell))?;
        if addr != ctx.accounts.craft_cell.key() {
            return Err(error!(CraftErr::Cell));
        }

        let log = &mut ctx.accounts.log;
        log.crafter = ctx.accounts.crafter.key();
        log.recipe = rec;
        log.energy_used = log.energy_used.saturating_add(e);
        log.cost_accum = log.cost_accum.wrapping_add(cost);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecordCraft<'info> {
    #[account(mut)]
    pub log: Account<'info, CraftLog>,
    /// CHECK:
    pub craft_cell: AccountInfo<'info>,
    pub crafter: AccountInfo<'info>,
}

#[account]
pub struct CraftLog {
    pub crafter: Pubkey,
    pub recipe: [u8; 12],
    pub energy_used: u32,
    pub cost_accum: u32,
}

#[error_code]
pub enum CraftErr {
    #[msg("Craft cell PDA mismatch")]
    Cell,
}
