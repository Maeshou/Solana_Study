// 7) Loot Forge Dist — 三角分布近似（PDAなし）
declare_id!("LFDT777777777777777777777777777777777");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ForgeState { Prep, Rolling, Jackpot }

#[program]
pub mod loot_forge_dist {
    use super::*;
    use ForgeState::*;

    pub fn init_forge(ctx: Context<InitForge>) -> Result<()> {
        let s = &mut ctx.accounts;
        s.board.owner = s.owner.key();
        s.board.state = Prep;
        Ok(())
    }

    pub fn roll(ctx: Context<RollLoot>, n: u32, period: u32) -> Result<()> {
        let s = &mut ctx.accounts;
        let per = period.max(5);

        for i in 0..n {
            let t = i % per;
            if t <= per/2 {
                let w = t * 2 + 3;
                s.mold.spark = s.mold.spark.wrapping_add(w);
                s.ledger.spikes = s.ledger.spikes.wrapping_add(w / 3 + 1);
                s.ledger.turns = s.ledger.turns.wrapping_add(1);
                msg!("triangle up");
            } else {
                let w = (per - t) * 2 + 3;
                s.mold.spark = s.mold.spark.wrapping_add(w);
                s.ledger.spikes = s.ledger.spikes.wrapping_add(w / 4 + 2);
                s.ledger.turns = s.ledger.turns.wrapping_add(1);
                msg!("triangle down");
            }
        }

        if s.mold.spark > 5_000 {
            s.board.state = Jackpot;
            s.ledger.badges = s.ledger.badges.wrapping_add(5);
            s.mold.spark = s.mold.spark / 2 + 111;
            s.ledger.spikes = s.ledger.spikes.wrapping_mul(2);
            msg!("jackpot: badges+5, spark half+111, spikes*2");
        } else {
            s.board.state = Rolling;
            s.ledger.turns = s.ledger.turns.wrapping_add(2);
            s.mold.spark = s.mold.spark + 77;
            s.ledger.spikes ^= 0x3333;
            msg!("rolling: turns+2, spark+77, spikes xor");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitForge<'info> {
    #[account(init, payer=payer, space=8+32+1)]
    pub board: Account<'info, LootBoard>,
    #[account(init, payer=payer, space=8+4)]
    pub mold: Account<'info, SparkMold>,
    #[account(init, payer=payer, space=8+8+4)]
    pub ledger: Account<'info, LootLedger>,
    #[account(mut)] pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RollLoot<'info> {
    #[account(mut, has_one=owner)]
    pub board: Account<'info, LootBoard>,
    #[account(
        mut,
        constraint = mold.key() != board.key() @ LfdErr::Dup,
        constraint = mold.key() != ledger.key() @ LfdErr::Dup
    )]
    pub mold: Account<'info, SparkMold>,
    #[account(
        mut,
        constraint = ledger.key() != board.key() @ LfdErr::Dup
    )]
    pub ledger: Account<'info, LootLedger>,
    pub owner: Signer<'info>,
}
#[account] pub struct LootBoard { pub owner: Pubkey, pub state: ForgeState }
#[account] pub struct SparkMold { pub spark: u32 }
#[account] pub struct LootLedger { pub turns: u64, pub spikes: u32, pub badges: u32 }
#[error_code] pub enum LfdErr { #[msg("dup")] Dup }