use anchor_lang::prelude::*;

declare_id!("BREED888888888888888888888888888888888888888");

#[program]
pub mod breeding_program {
    use super::*;
    /// 2体のNFTペットを配合する
    pub fn breed_pets(ctx: Context<BreedPets>) -> Result<()> {
        let parent_a = &mut ctx.accounts.parent_a;
        let parent_b = &mut ctx.accounts.parent_b;
        let clock = Clock::get()?;
        
        parent_a.breed_count = parent_a.breed_count.saturating_add(1);
        parent_b.breed_count = parent_b.breed_count.saturating_add(1);
        parent_a.last_breed_timestamp = clock.unix_timestamp;
        parent_b.last_breed_timestamp = clock.unix_timestamp;

        // 子のステータスを計算（実際には新しいNFTをミントするCPIを呼び出す）
        let egg_strength = (parent_a.stats.strength + parent_b.stats.strength) / 2;
        msg!("Breeding successful! New egg strength will be {}.", egg_strength);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BreedPets<'info> {
    #[account(mut, has_one = owner, constraint = parent_a.breed_count < 5)]
    pub parent_a: Account<'info, PetNft>,
    #[account(mut, has_one = owner, constraint = parent_b.breed_count < 5)]
    pub parent_b: Account<'info, PetNft>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct PetNft {
    pub owner: Pubkey,
    pub stats: PetStats,
    pub breed_count: u32,
    pub last_breed_timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct PetStats {
    pub strength: u32,
    pub intelligence: u32,
}