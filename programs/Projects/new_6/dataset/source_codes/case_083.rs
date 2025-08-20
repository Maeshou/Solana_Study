use anchor_lang::prelude::*;

declare_id!("CraftStation11111111111111111111111111111111");

#[program]
pub mod nft_craft_station_mixup {
    use super::*;

    pub fn combine_parts(ctx: Context<CombineParts>, bonus_points: u16) -> Result<()> {
        let raw = &ctx.accounts.user_config;
        let user_data = CraftUser::try_from_slice(&raw.data.borrow())?;

        if user_data.creator != ctx.accounts.creator.key() {
            return Err(ProgramError::InvalidAccountData.into());
        }

        let forge = &mut ctx.accounts.nft_forge;

        forge.used_slots += 1;
        forge.accumulated_bonus += bonus_points;
        forge.combinations.push((
            Clock::get()?.unix_timestamp,
            user_data.creator,
            bonus_points,
        ));

        for _ in 0..2 {
            forge.random_ids.push(rand::random::<u16>());
        }

        if forge.used_slots > 20 {
            forge.system_note = Some("Too many combines".to_string());
            forge.penalty_mode = true;
        }

        forge.last_active = Clock::get()?.slot;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CombineParts<'info> {
    pub creator: Signer<'info>,
    /// CHECK: Can be structurally misused
    pub user_config: AccountInfo<'info>,
    #[account(mut)]
    pub nft_forge: Account<'info, Forge>,
}

#[account]
pub struct Forge {
    pub used_slots: u16,
    pub accumulated_bonus: u16,
    pub combinations: Vec<(i64, Pubkey, u16)>,
    pub random_ids: Vec<u16>,
    pub system_note: Option<String>,
    pub penalty_mode: bool,
    pub last_active: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CraftUser {
    pub creator: Pubkey,
    pub score: u32,
}
