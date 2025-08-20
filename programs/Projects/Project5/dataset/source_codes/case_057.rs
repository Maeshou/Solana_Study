// ============================================================================
// 2) Relic Foundry — 遺物鋳造（PDAなし / has_one + lane不一致）
// ============================================================================
declare_id!("RLFD22222222222222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum FoundryState { Prep, Pour, Cool }

#[program]
pub mod relic_foundry {
    use super::*;
    use FoundryState::*;

    pub fn init_foundry(ctx: Context<InitFoundry>, quota: u32) -> Result<()> {
        let f = &mut ctx.accounts;
        f.foundry.owner = f.owner.key();
        f.foundry.quota = quota;
        f.foundry.state = Prep;

        f.core.foundry = f.foundry.key(); f.core.lane = 10;
        f.mold.foundry = f.foundry.key(); f.mold.lane = 20;
        f.book.foundry = f.foundry.key(); f.book.lane = 99;
        Ok(())
    }

    pub fn cast(ctx: Context<Cast>, cycles: u32) -> Result<()> {
        let f = &mut ctx.accounts;

        for i in 0..cycles {
            f.core.heat = f.core.heat.checked_add(6 + (i % 4)).unwrap_or(u32::MAX);
            f.mold.stress = f.mold.stress.saturating_add(5 + (i % 3));
            f.book.units = (f.book.units as u128 + 3 + (i as u128 % 5)).min(u128::from(u64::MAX)) as u64;
        }

        let load = f.core.heat as u64 + f.mold.stress as u64;
        if load > f.foundry.quota as u64 {
            f.foundry.state = Cool;
            f.book.defects = f.book.defects.saturating_add(2);
            f.mold.stress = f.mold.stress / 2 + 8;
            f.core.heat = f.core.heat / 2 + 6;
            msg!("cool: defects+2, stress/heat damp");
        } else {
            f.foundry.state = Pour;
            f.book.units = f.book.units.saturating_add(11);
            f.core.heat ^= 0x55AA_33CC;
            f.mold.stress = f.mold.stress.saturating_add(7);
            msg!("pour: units+11, core xor, mold+7");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFoundry<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub foundry: Account<'info, Foundry>,
    #[account(init, payer=payer, space=8+32+1+4)]
    pub core: Account<'info, CoreLine>,
    #[account(init, payer=payer, space=8+32+1+4)]
    pub mold: Account<'info, MoldLine>,
    #[account(init, payer=payer, space=8+32+1+8+4)]
    pub book: Account<'info, FoundryBook>,
    #[account(mut)] pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Cast<'info> {
    #[account(mut, has_one=owner)]
    pub foundry: Account<'info, Foundry>,
    #[account(mut, has_one=foundry, constraint = core.lane != mold.lane @ RfErr::Dup)]
    pub core: Account<'info, CoreLine>,
    #[account(mut, has_one=foundry, constraint = mold.lane != book.lane @ RfErr::Dup)]
    pub mold: Account<'info, MoldLine>,
    #[account(mut, has_one=foundry)]
    pub book: Account<'info, FoundryBook>,
    pub owner: Signer<'info>,
}

#[account] pub struct Foundry { pub owner: Pubkey, pub quota: u32, pub state: FoundryState }
#[account] pub struct CoreLine { pub foundry: Pubkey, pub lane: u8, pub heat: u32 }
#[account] pub struct MoldLine { pub foundry: Pubkey, pub lane: u8, pub stress: u32 }
#[account] pub struct FoundryBook { pub foundry: Pubkey, pub lane: u8, pub units: u64, pub defects: u32 }
#[error_code] pub enum RfErr { #[msg("dup")] Dup }

