use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA33mvTWf");

#[program]
pub mod battle_stat_tracker_003 {
    use super::*;

    pub fn record_battle(ctx: Context<Ctx003>, amount: u64) -> Result<()> {
        let p = &mut ctx.accounts.player_data;

        let max_candidate = amount > p.max_damage;
        p.max_damage = (max_candidate as u64) * amount + (!max_candidate as u64) * p.max_damage;

        p.total_damage += amount;
        p.battle_count += 1;

        Ok(())
    }

    pub fn show_stats(ctx: Context<Ctx003>) -> Result<()> {
        let p = &ctx.accounts.player_data;
        let average = p.total_damage / (p.battle_count + (p.battle_count == 0) as u64); // ゼロ除算回避

        msg!("Total Damage: {}", p.total_damage);
        msg!("Max Damage: {}", p.max_damage);
        msg!("Battle Count: {}", p.battle_count);
        msg!("Average Damage: {}", average);

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
    pub total_damage: u64,
    pub max_damage: u64,
    pub battle_count: u64,
}
