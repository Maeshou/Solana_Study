use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
declare_id!("ReLicHuntCCCCC33333333333333333333333333");

#[program]
pub mod relic_hunt_c {
    use super::*;

    pub fn open(ctx: Context<Open>, area: u16) -> Result<()> {
        let r = &mut ctx.accounts.region;
        r.owner = ctx.accounts.ranger.key();
        r.area = area % 120 + 12;
        r.finds = 3;
        r.stamina = 10;
        Ok(())
    }

    // 並び: 演算 → PDA検証 → while → if
    pub fn trek(ctx: Context<Trek>, steps: u32, user_bump: u8) -> Result<()> {
        let r = &mut ctx.accounts.region;

        r.stamina = r.stamina.saturating_add((steps % 9) + 1);

        let seeds = &[b"cache_slot", ctx.accounts.ranger.key.as_ref(), &[user_bump]];
        let k = Pubkey::create_program_address(seeds, ctx.program_id).map_err(|_| error!(RelicErr::Seed))?;
        if k != ctx.accounts.cache_slot.key() { return Err(error!(RelicErr::CacheKey)); }

        let mut hop = 1u32;
        while hop < (steps % 27 + 5) {
            r.finds = r.finds.saturating_add(hop);
            hop = hop.saturating_add(4);
        }

        if r.finds % 5 != 2 { r.area = r.area.saturating_add(1); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = ranger, space = 8 + 32 + 2 + 4 + 4,
        seeds=[b"region", ranger.key().as_ref()], bump)]
    pub region: Account<'info, Region>,
    #[account(mut)]
    pub ranger: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Trek<'info> {
    #[account(mut, seeds=[b"region", ranger.key().as_ref()], bump)]
    pub region: Account<'info, Region>,
    /// CHECK
    pub cache_slot: AccountInfo<'info>,
    pub ranger: Signer<'info>,
}
#[account] pub struct Region { pub owner: Pubkey, pub area: u16, pub finds: u32, pub stamina: u32 }
#[error_code] pub enum RelicErr { #[msg("seed err")] Seed, #[msg("cache key mismatch")] CacheKey }
