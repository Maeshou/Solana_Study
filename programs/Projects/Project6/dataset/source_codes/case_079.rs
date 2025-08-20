use anchor_lang::prelude::*;

declare_id!("RESRC999999999999999999999999999999999999999");

#[program]
pub mod resource_generator_program {
    use super::*;
    /// 施設が時間経過で生成した資源を回収します。
    pub fn generate_passive_resources(ctx: Context<GenerateResources>) -> Result<()> {
        let generator = &mut ctx.accounts.resource_generator;
        let player_resources = &mut ctx.accounts.player_resources;
        let clock = Clock::get()?;

        let time_elapsed = clock.unix_timestamp.saturating_sub(generator.last_claim_timestamp);
        let gold_generated = (time_elapsed as u64).saturating_mul(generator.gold_per_second);

        player_resources.gold = player_resources.gold.saturating_add(gold_generated);
        generator.last_claim_timestamp = clock.unix_timestamp;

        msg!("Claimed {} gold.", gold_generated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GenerateResources<'info> {
    #[account(mut, has_one = owner)]
    pub resource_generator: Account<'info, ResourceGenerator>,
    #[account(mut, has_one = owner)]
    pub player_resources: Account<'info, PlayerResources>,
    pub owner: Signer<'info>,
}

#[account]
pub struct ResourceGenerator {
    pub owner: Pubkey,
    pub gold_per_second: u64,
    pub last_claim_timestamp: i64,
}

#[account]
pub struct PlayerResources {
    pub owner: Pubkey,
    pub gold: u64,
}