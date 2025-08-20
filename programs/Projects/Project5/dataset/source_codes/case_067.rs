use anchor_lang::prelude::*;

// ======================================================================
// 1) Card Table Croupier: トランプ卓の配りとポット管理（LFSR, ローリングチェックサム）
// ======================================================================
declare_id!("CRPR111111111111111111111111111111111111111");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TableMode { Closed, Dealing, Settled }

#[program]
pub mod card_table_croupier {
    use super::*;
    use TableMode::*;

    pub fn init_table(ctx: Context<InitTable>, pot_seed: u64) -> Result<()> {
        let t = &mut ctx.accounts.parent;
        let a = &mut ctx.accounts.seat_a;
        let b = &mut ctx.accounts.seat_b;
        let l = &mut ctx.accounts.ledger;

        t.owner = ctx.accounts.owner.key();
        t.pot = pot_seed;
        t.mode = Closed;

        a.parent = t.key();
        a.seat_id = 1;
        a.chips = 100;
        a.phase = 0;

        b.parent = t.key();
        b.seat_id = 2;
        b.chips = 120;
        b.phase = 0;

        l.parent = t.key();
        l.ring = 9;
        l.lfsr = 0xA5A5_1234;
        l.check = 0;

        Ok(())
    }

    pub fn deal_round(ctx: Context<DealRound>, rounds: u32) -> Result<()> {
        let t = &mut ctx.accounts.parent;
        let a = &mut ctx.accounts.seat_a;
        let b = &mut ctx.accounts.seat_b;
        let l = &mut ctx.accounts.ledger;

        for r in 0..rounds {
            // 32-bit LFSR (taps 32,30,26,25)
            let bit = ((l.lfsr >> 0) ^ (l.lfsr >> 2) ^ (l.lfsr >> 6) ^ (l.lfsr >> 7)) & 1;
            l.lfsr = (l.lfsr >> 1) | (bit << 31);

            let draw_a = (l.lfsr & 31) as u64;
            let draw_b = ((l.lfsr >> 5) & 31) as u64;

            a.chips = a.chips.checked_add(draw_a).unwrap_or(u64::MAX);
            b.chips = b.chips.saturating_sub(draw_b.min(b.chips));
            t.pot = t.pot.saturating_add(draw_b);
            l.check = l.check.wrapping_add(((a.chips ^ b.chips) as u64).wrapping_mul(1315423911));
        }

        let swing = (a.chips as i128 - b.chips as i128).unsigned_abs();
        if swing > t.pot {
            t.mode = Settled;
            a.phase = a.phase.saturating_add(3);
            b.phase = b.phase.saturating_add(1);
            l.check ^= (t.pot.rotate_left(13)) as u64;
            msg!("settled: phases bumped, check mixed");
        } else {
            t.mode = Dealing;
            a.chips = a.chips.saturating_add(7);
            b.chips = b.chips.saturating_add(5);
            l.lfsr ^= 0x00FF_F0F0;
            msg!("dealing: small chip boosts, lfsr tweak");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTable<'info> {
    #[account(init, payer=payer, space=8 + 32 + 8 + 1)]
    pub parent: Account<'info, TableCore>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub seat_a: Account<'info, SeatBox>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub seat_b: Account<'info, SeatBox>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4 + 8)]
    pub ledger: Account<'info, TableLedger>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DealRound<'info> {
    #[account(mut, has_one=owner)]
    pub parent: Account<'info, TableCore>,
    #[account(
        mut,
        has_one=parent,
        constraint = seat_a.seat_id != seat_b.seat_id @ CroupierErr::Dup
    )]
    pub seat_a: Account<'info, SeatBox>,
    #[account(
        mut,
        has_one=parent,
        constraint = seat_b.seat_id != ledger.ring @ CroupierErr::Dup
    )]
    pub seat_b: Account<'info, SeatBox>,
    #[account(mut, has_one=parent)]
    pub ledger: Account<'info, TableLedger>,
    pub owner: Signer<'info>,
}

#[account]
pub struct TableCore {
    pub owner: Pubkey,
    pub pot: u64,
    pub mode: TableMode,
}

#[account]
pub struct SeatBox {
    pub parent: Pubkey,
    pub seat_id: u8,
    pub chips: u64,
    pub phase: u32,
}

#[account]
pub struct TableLedger {
    pub parent: Pubkey,
    pub ring: u8,
    pub lfsr: u32,
    pub check: u64,
}

#[error_code]
pub enum CroupierErr {
    #[msg("duplicate mutable account")]
    Dup,
}
