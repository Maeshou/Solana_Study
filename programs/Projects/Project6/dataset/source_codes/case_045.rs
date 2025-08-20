// 8) beacon_territory_mini: 領地ビーコンの軽量版
use anchor_lang::prelude::*;
declare_id!("BeAcOnTeRrItOrYmInI1111111111111111111");

#[program]
pub mod beacon_territory_mini {
    use super::*;

    pub fn init_beacon(ctx: Context<InitBeacon>, strength: u32) -> Result<()> {
        let b = &mut ctx.accounts.beacon;
        b.lord = ctx.accounts.lord.key();
        b.strength = strength;
        b.depth = 5;
        Ok(())
    }

    pub fn ignite(ctx: Context<Ignite>, shards: u64, phase_index: u8) -> Result<()> {
        let b = &mut ctx.accounts.beacon;

        if b.depth < 20 {
            let mut i = 0u8;
            while i < 3 {
                b.depth = b.depth.saturating_add(((i as u32) << 1).saturating_add(1));
                i = i.saturating_add(1);
            }
        }

        let coef: [u64; 3] = [90, 110, 140];
        let idx = if phase_index > 2 { 2 } else { phase_index } as usize;
        let mut inc = shards.saturating_mul(coef[idx]);

        let mut r = 0u8;
        while r < 4 {
            inc = inc.rotate_left(1).wrapping_add(13);
            r = r.saturating_add(1);
        }

        b.strength = b.strength.saturating_add((inc % 37) as u32);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBeacon<'info> {
    #[account(init, payer = lord, space = 8 + 32 + 4 + 4)]
    pub beacon: Account<'info, BeaconState>,
    #[account(mut)] pub lord: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Ignite<'info> {
    #[account(mut, has_one = lord)]
    pub beacon: Account<'info, BeaconState>,
    pub lord: Signer<'info>,
}
#[account]
pub struct BeaconState {
    pub lord: Pubkey,
    pub strength: u32,
    pub depth: u32,
}
