// ======================================================================
// 7) Bee Colony：蜂群（初期化＝ハニーフレーム配分）
// ======================================================================
declare_id!("BEEE7777777777777777777777777777777777777");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum HiveStage { Build, Harvest, Winter }

#[program]
pub mod bee_colony {
    use super::*;
    use HiveStage::*;

    pub fn init_hive(ctx: Context<InitHive>, nectar: u32) -> Result<()> {
        let h = &mut ctx.accounts.hive;
        h.owner = ctx.accounts.keeper.key();
        h.nectar = nectar;
        h.stage = Build;

        let a = &mut ctx.accounts.frame_a;
        let b = &mut ctx.accounts.frame_b;
        let lg = &mut ctx.accounts.ledger;

        a.hive = h.key(); a.cell = (nectar & 7) as u8; a.honey = nectar + 13;
        b.hive = h.key(); b.cell = ((nectar >> 2) & 7) as u8; b.honey = nectar.rotate_left(3) + 7;

        lg.hive = h.key(); lg.cell = 9; lg.frames = 0; lg.chaos = nectar as u64 ^ 0x55AA_33CC;
        Ok(())
    }

    pub fn forage(ctx: Context<Forage>, days: u32) -> Result<()> {
        let h = &mut ctx.accounts.hive;
        let a = &mut ctx.accounts.frame_a;
        let b = &mut ctx.accounts.frame_b;
        let lg = &mut ctx.accounts.ledger;

        for i in 0..days {
            let r = ((a.honey ^ b.honey) as u64).wrapping_mul(11400714819323198485);
            a.honey = a.honey.checked_add(((r & 31) as u32) + 2).unwrap_or(u32::MAX);
            b.honey = b.honey.saturating_add((((r >> 5) & 31) as u32) + 3);
            lg.frames = lg.frames.saturating_add(1);
            lg.chaos ^= r.rotate_left((i % 13) as u32);
        }

        let total = a.honey + b.honey;
        if total > h.nectar * 5 {
            h.stage = Winter;
            a.cell ^= 1; b.cell = b.cell.saturating_add(1);
            lg.cell = lg.cell.saturating_add(1);
            msg!("winter: cell tweaks & ledger move");
        } else {
            h.stage = Harvest;
            a.honey = a.honey.saturating_add(9);
            b.honey = b.honey / 2 + 11;
            lg.chaos ^= 0x0F0F_F0F0;
            msg!("harvest: adjust honey & chaos flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHive<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub hive: Account<'info, Hive>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub frame_a: Account<'info, Frame>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub frame_b: Account<'info, Frame>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub ledger: Account<'info, HiveLog>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub keeper: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Forage<'info> {
    #[account(mut, has_one=owner)]
    pub hive: Account<'info, Hive>,
    #[account(
        mut,
        has_one=hive,
        constraint = frame_a.cell != frame_b.cell @ BeeErr::Dup
    )]
    pub frame_a: Account<'info, Frame>,
    #[account(
        mut,
        has_one=hive,
        constraint = frame_b.cell != ledger.cell @ BeeErr::Dup
    )]
    pub frame_b: Account<'info, Frame>,
    #[account(mut, has_one=hive)]
    pub ledger: Account<'info, HiveLog>,
    pub keeper: Signer<'info>,
}

#[account] pub struct Hive { pub owner: Pubkey, pub nectar: u32, pub stage: HiveStage }
#[account] pub struct Frame{ pub hive: Pubkey, pub cell: u8, pub honey: u32 }
#[account] pub struct HiveLog{ pub hive: Pubkey, pub cell: u8, pub frames: u64, pub chaos: u64 }

#[error_code] pub enum BeeErr { #[msg("duplicate mutable account")] Dup }
