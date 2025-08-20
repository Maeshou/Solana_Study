// 2) Potion Brewery FP — Q32.32 抽出（PDAなし）
declare_id!("PTFP222222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BrewStage { Mash, Boil, Age }

#[program]
pub mod potion_brewery_fp {
    use super::*;
    use BrewStage::*;

    pub fn init_brew(ctx: Context<InitBrew>, base_q32: u64) -> Result<()> {
        let a = &mut ctx.accounts;
        a.house.alchemist = a.alchemist.key();
        a.house.base_q32 = base_q32;
        a.house.stage = Mash;
        Ok(())
    }

    pub fn infuse(ctx: Context<Infuse>, rounds: u32) -> Result<()> {
        let a = &mut ctx.accounts;

        for _ in 0..rounds {
            a.kettle.essence_q32 = a.kettle.essence_q32.wrapping_add(a.house.base_q32 >> 6);
            a.kettle.density_q32 = ((u128::from(a.kettle.density_q32) * 5) / 4).min(u128::from(u64::MAX)) as u64;
            a.kettle.tint = a.kettle.tint.rotate_left(3) ^ (a.kettle.tint >> 2);
            a.ledger.casks = a.ledger.casks.wrapping_add(1);
        }

        if (a.kettle.essence_q32 >> 32) > 200 {
            a.house.stage = Age;
            a.ledger.notes = a.ledger.notes.wrapping_add(3);
            a.kettle.density_q32 = a.kettle.density_q32 + (1u64<<31);
            a.kettle.tint ^= 0x6D6D_AAAA;
            msg!("aging: notes+3, density+0.5, tint xor");
        } else {
            a.house.stage = Boil;
            a.ledger.casks = a.ledger.casks.wrapping_mul(2);
            a.kettle.essence_q32 = a.kettle.essence_q32 + (1u64<<32);
            a.kettle.tint = a.kettle.tint.wrapping_add(11);
            msg!("boiling: casks*2, essence+1.0, tint+11");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBrew<'info> {
    #[account(init, payer=payer, space=8+32+8+1)]
    pub house: Account<'info, BrewHouse>,
    #[account(init, payer=payer, space=8+8+8+4)]
    pub kettle: Account<'info, KettleQ32>,
    #[account(init, payer=payer, space=8+4+4)]
    pub ledger: Account<'info, BrewLedger>,
    #[account(mut)] pub payer: Signer<'info>,
    pub alchemist: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Infuse<'info> {
    #[account(mut, has_one=alchemist)]
    pub house: Account<'info, BrewHouse>,
    #[account(
        mut,
        constraint = kettle.key() != house.key() @ PtfpErr::Dup,
        constraint = kettle.key() != ledger.key() @ PtfpErr::Dup
    )]
    pub kettle: Account<'info, KettleQ32>,
    #[account(
        mut,
        constraint = ledger.key() != house.key() @ PtfpErr::Dup
    )]
    pub ledger: Account<'info, BrewLedger>,
    pub alchemist: Signer<'info>,
}
#[account] pub struct BrewHouse { pub alchemist: Pubkey, pub base_q32: u64, pub stage: BrewStage }
#[account] pub struct KettleQ32 { pub essence_q32: u64, pub density_q32: u64, pub tint: u32 }
#[account] pub struct BrewLedger { pub casks: u32, pub notes: u32 }
#[error_code] pub enum PtfpErr { #[msg("dup")] Dup }
