// ===============================================
// (3) raid_board: レイド記録（偵察票・結果板・統計）
// ===============================================
use anchor_lang::prelude::*;
declare_id!("RA1dBoArD3333333333333333333333333333333");

#[program]
pub mod raid_board {
    use super::*;

    pub fn init_scout(ctx: Context<InitScout>, seat: u8) -> Result<()> {
        let s = &mut ctx.accounts.scout;
        s.parent = ctx.accounts.result_board.key();
        s.seat = seat;
        s.power = 0;
        Ok(())
    }

    pub fn init_result(ctx: Context<InitResult>) -> Result<()> {
        let r = &mut ctx.accounts.result_board;
        let st = &mut ctx.accounts.stats;
        r.owner = ctx.accounts.owner.key();
        r.last = 0;
        r.hist = [0; 6];

        st.owner = ctx.accounts.owner.key();
        st.win = 0;
        st.lose = 0;
        st.score = 0;
        Ok(())
    }

    pub fn process_raid(ctx: Context<ProcessRaid>, energy: u32) -> Result<()> {
        let a = &mut ctx.accounts.scout_a;
        let b = &mut ctx.accounts.scout_b;
        let r = &mut ctx.accounts.result_board;
        let st = &mut ctx.accounts.stats;

        // ループ（6 ラウンド）
        for i in 0..6 {
            let e = (energy as u64).rotate_left((i as u32) & 31) ^ (a.power ^ b.power);
            let gain = ((e & 0xFFFF) as u32) % 1000;
            r.hist[i] = r.hist[i].saturating_add(gain);
            r.last = r.hist[i];
        }

        if (r.last & 1) == 0 {
            // True: A 側強化
            a.power = a.power.saturating_add((r.last / 3) as u64);
            st.win = st.win.saturating_add(1);
            st.score = st.score.saturating_add(r.last as u64);
            msg!("A boosted");
        } else {
            // False: B 側強化
            b.power = b.power.saturating_add((r.last / 2) as u64);
            st.lose = st.lose.saturating_add(1);
            st.score = st.score.saturating_add((r.last / 2) as u64);
            msg!("B boosted");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitResult<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4*6)]
    pub result_board: Account<'info, ResultBoard>,
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub stats: Account<'info, RaidStats>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitScout<'info> {
    #[account(mut)]
    pub result_board: Account<'info, ResultBoard>,
    #[account(init, payer = owner, space = 8 + 32 + 1 + 8)]
    pub scout: Account<'info, Scout>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessRaid<'info> {
    #[account(mut)]
    pub result_board: Account<'info, ResultBoard>,
    #[account(
        mut,
        constraint = scout_a.parent == result_board.key() @ RaidErr::Cosplay,
        constraint = scout_a.seat != scout_b.seat @ RaidErr::Cosplay
    )]
    pub scout_a: Account<'info, Scout>,
    #[account(
        mut,
        constraint = scout_b.parent == result_board.key() @ RaidErr::Cosplay
    )]
    pub scout_b: Account<'info, Scout>,
    #[account(mut)]
    pub stats: Account<'info, RaidStats>,
}

#[account]
pub struct ResultBoard {
    pub owner: Pubkey,
    pub last: u32,
    pub hist: [u32; 6],
}

#[account]
pub struct Scout {
    pub parent: Pubkey, // = result_board
    pub seat: u8,
    pub power: u64,
}

#[account]
pub struct RaidStats {
    pub owner: Pubkey,
    pub win: u64,
    pub lose: u64,
    pub score: u64,
}

#[error_code]
pub enum RaidErr { #[msg("cosplay blocked")] Cosplay }
