// ============================================================================
// 4) Coral Farm — サンゴ育成（PDAなし / has_one + slot不一致）
// ============================================================================
declare_id!("CRFM44444444444444444444444444444444444444444");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ReefState { Calm, Feed, Bloom }

#[program]
pub mod coral_farm {
    use super::*;
    use ReefState::*;

    pub fn init_reef(ctx: Context<InitReef>, cap: u32) -> Result<()> {
        let r = &mut ctx.accounts;
        r.pool.keeper = r.keeper.key();
        r.pool.cap = cap;
        r.pool.state = Calm;

        r.pet.pool = r.pool.key(); r.pet.slot = 1;
        r.feeder.pool = r.pool.key(); r.feeder.slot = 2;
        r.log.pool = r.pool.key(); r.log.slot = 9;
        Ok(())
    }

    pub fn feed(ctx: Context<Feed>, scoops: u32) -> Result<()> {
        let r = &mut ctx.accounts;

        for i in 0..scoops {
            r.pet.size = r.pet.size.checked_add(6 + (i % 4)).unwrap_or(u32::MAX);
            r.feeder.stock = r.feeder.stock.saturating_add(3 + (i % 3));
            r.log.events = r.log.events.saturating_add(1);
        }

        if r.pet.size as u64 > r.pool.cap as u64 {
            r.pool.state = Bloom;
            r.log.bonus = r.log.bonus.saturating_add(2);
            r.feeder.stock = r.feeder.stock / 2 + 5;
            r.pet.size = r.pet.size / 2 + 7;
            msg!("bloom: bonus+2, damp size/stock");
        } else {
            r.pool.state = Feed;
            r.log.events = r.log.events.saturating_add(3);
            r.pet.size = r.pet.size.saturating_add(9);
            r.feeder.stock = r.feeder.stock.saturating_add(7);
            msg!("feed: events+3, size+9, stock+7");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitReef<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub pool: Account<'info, Pool>,
    #[account(init, payer=payer, space=8+32+1+4)]
    pub pet: Account<'info, Pet>,
    #[account(init, payer=payer, space=8+32+1+4)]
    pub feeder: Account<'info, Feeder>,
    #[account(init, payer=payer, space=8+32+1+8+4)]
    pub log: Account<'info, ReefLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub keeper: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Feed<'info> {
    #[account(mut, has_one=keeper)]
    pub pool: Account<'info, Pool>,
    #[account(mut, has_one=pool, constraint = pet.slot != feeder.slot @ CfErr::Dup)]
    pub pet: Account<'info, Pet>,
    #[account(mut, has_one=pool, constraint = feeder.slot != log.slot @ CfErr::Dup)]
    pub feeder: Account<'info, Feeder>,
    #[account(mut, has_one=pool)]
    pub log: Account<'info, ReefLog>,
    pub keeper: Signer<'info>,
}

#[account] pub struct Pool { pub keeper: Pubkey, pub cap: u32, pub state: ReefState }
#[account] pub struct Pet { pub pool: Pubkey, pub slot: u8, pub size: u32 }
#[account] pub struct Feeder { pub pool: Pubkey, pub slot: u8, pub stock: u32 }
#[account] pub struct ReefLog { pub pool: Pubkey, pub slot: u8, pub events: u64, pub bonus: u32 }
#[error_code] pub enum CfErr { #[msg("dup")] Dup }