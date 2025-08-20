use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("Ladder0666666666666666666666666666666666");

#[program]
pub mod challenge_ladder {
    use super::*;

    pub fn submit_result(ctx: Context<Submit>, challenger: Pubkey, opponent: Pubkey, won: bool) -> Result<()> {
        let ld = &mut ctx.accounts.ladder;
        if won {
            *ld.points.entry(challenger).or_insert(0) += 10;
            ld.losses.entry(opponent).and_modify(|c| *c += 1).or_insert(1);
        } else {
            *ld.points.entry(opponent).or_insert(0) += 5;
            ld.losses.entry(challenger).and_modify(|c| *c += 1).or_insert(1);
        }
        ld.matches += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Submit<'info> {
    #[account(mut)]
    pub ladder: Account<'info, LadderData>,
}

#[account]
pub struct LadderData {
    pub points: BTreeMap<Pubkey, u64>,
    pub losses: BTreeMap<Pubkey, u64>,
    pub matches: u64,
}
