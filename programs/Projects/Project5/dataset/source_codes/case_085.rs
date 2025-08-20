// ======================================================================
// 8) Reader Lounge：読書室（初期化＝逐次混合で初期混合値）
// ======================================================================
declare_id!("READ88888888888888888888888888888888888888");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LoungeState { Queue, Read, Close }

#[program]
pub mod reader_lounge {
    use super::*;
    use LoungeState::*;

    pub fn init_lounge(ctx: Context<InitLounge>, base: u32) -> Result<()> {
        let l = &mut ctx.accounts.lounge;
        l.owner = ctx.accounts.librarian.key();
        l.max = base * 4 + 100;
        l.state = Queue;

        let a = &mut ctx.accounts.reader_a;
        let b = &mut ctx.accounts.reader_b;
        let t = &mut ctx.accounts.tape;

        a.lounge = l.key(); a.seat = (base & 7) as u8; a.pages = base + 9;
        b.lounge = l.key(); b.seat = ((base >> 2) & 7) as u8; b.pages = base.rotate_left(2) + 11;

        t.lounge = l.key(); t.seat = 9; t.count = 0; t.mix = (base as u64) ^ 0xFACE_1234;
        Ok(())
    }

    pub fn sit(ctx: Context<Sit>, laps: u32) -> Result<()> {
        let l = &mut ctx.accounts.lounge;
        let a = &mut ctx.accounts.reader_a;
        let b = &mut ctx.accounts.reader_b;
        let t = &mut ctx.accounts.tape;

        for i in 0..laps {
            let mix = ((a.pages ^ b.pages) as u64).wrapping_mul(2654435761);
            a.pages = a.pages.checked_add(((mix & 31) as u32) + 2).unwrap_or(u32::MAX);
            b.pages = b.pages.saturating_add((((mix >> 5) & 31) as u32) + 3);
            t.count = t.count.saturating_add(1);
            t.mix ^= mix.rotate_left((i % 13) as u32);
        }

        let mean = if t.count == 0 { 0 } else { (t.mix / t.count) as u32 };
        if mean > l.max {
            l.state = Close;
            a.seat ^= 1; b.seat = b.seat.saturating_add(1);
            t.seat = t.seat.saturating_add(1);
            msg!("close: seat tweaks & tape move");
        } else {
            l.state = Read;
            a.pages = a.pages.saturating_add(9);
            b.pages = b.pages / 2 + 11;
            t.mix ^= 0x0F0F_F0F0;
            msg!("read: adjust pages & mix flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLounge<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub lounge: Account<'info, Lounge>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub reader_a: Account<'info, Reader>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub reader_b: Account<'info, Reader>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub tape: Account<'info, ReadTape>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub librarian: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Sit<'info> {
    #[account(mut, has_one=owner)]
    pub lounge: Account<'info, Lounge>,
    #[account(
        mut,
        has_one=lounge,
        constraint = reader_a.seat != reader_b.seat @ ReadErr::Dup
    )]
    pub reader_a: Account<'info, Reader>,
    #[account(
        mut,
        has_one=lounge,
        constraint = reader_b.seat != tape.seat @ ReadErr::Dup
    )]
    pub reader_b: Account<'info, Reader>,
    #[account(mut, has_one=lounge)]
    pub tape: Account<'info, ReadTape>,
    pub librarian: Signer<'info>,
}

#[account] pub struct Lounge  { pub owner: Pubkey, pub max: u32, pub state: LoungeState }
#[account] pub struct Reader  { pub lounge: Pubkey, pub seat: u8, pub pages: u32 }
#[account] pub struct ReadTape{ pub lounge: Pubkey, pub seat: u8, pub count: u64, pub mix: u64 }

#[error_code] pub enum ReadErr { #[msg("duplicate mutable account")] Dup }
