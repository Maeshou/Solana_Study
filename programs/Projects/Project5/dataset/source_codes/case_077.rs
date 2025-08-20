// ======================================================================
// 10) Dojo Training：稽古場（初期化=位相をズラして経験値配分）
// ======================================================================
declare_id!("DOJO10101010101010101010101010101010101010");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Drill { Warm, Spar, Rest }

#[program]
pub mod dojo_training {
    use super::*;
    use Drill::*;

    pub fn init_dojo(ctx: Context<InitDojo>, base: u32) -> Result<()> {
        let d = &mut ctx.accounts.dojo;
        d.owner = ctx.accounts.master.key();
        d.ceiling = base * 5 + 80;
        d.flow = Warm;

        let s = &mut ctx.accounts.student_a;
        let t = &mut ctx.accounts.student_b;
        let b = &mut ctx.accounts.board;

        // 異なる位相で経験値配分
        s.parent = d.key(); s.mat = (base & 7) as u8; s.exp = (base * 3 + 11) % 257;
        t.parent = d.key(); t.mat = ((base >> 1) & 7) as u8; t.exp = (base * 5 + 17) % 263;

        b.parent = d.key(); b.ring = 9; b.count = 0; b.score = (base as u64) ^ 0xABCD_EF01;
        Ok(())
    }

    pub fn spar(ctx: Context<Spar>, r: u32) -> Result<()> {
        let d = &mut ctx.accounts.dojo;
        let s = &mut ctx.accounts.student_a;
        let t = &mut ctx.accounts.student_b;
        let b = &mut ctx.accounts.board;

        for i in 0..r {
            let phase = ((i as i32 - 5).abs() as u32) + 1;
            s.exp = s.exp.checked_add(phase + (i & 3)).unwrap_or(u32::MAX);
            t.exp = t.exp.saturating_add((phase / 2) + 2);
            b.count = b.count.saturating_add(1);
            b.score ^= ((s.exp as u64) << (i % 7)) ^ ((t.exp as u64) << (i % 11));
        }

        let avg = if b.count == 0 { 0 } else { (b.score / b.count) as u32 };
        if avg > d.ceiling {
            d.flow = Rest;
            s.mat ^= 1; t.mat = t.mat.saturating_add(1);
            b.ring = b.ring.saturating_add(1);
            msg!("rest: ring++ & mat tweaks");
        } else {
            d.flow = Spar;
            s.exp = s.exp.saturating_add(9);
            t.exp = t.exp / 2 + 7;
            b.score ^= 0x0FF0_FF0F;
            msg!("spar: exp adjust & score flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDojo<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub dojo: Account<'info, Dojo>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub student_a: Account<'info, Student>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub student_b: Account<'info, Student>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub board: Account<'info, BoardTape>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Spar<'info> {
    #[account(mut, has_one=owner)]
    pub dojo: Account<'info, Dojo>,
    #[account(
        mut,
        has_one=dojo,
        constraint = student_a.mat != student_b.mat @ DojoErr::Dup
    )]
    pub student_a: Account<'info, Student>,
    #[account(
        mut,
        has_one=dojo,
        constraint = student_b.mat != board.ring @ DojoErr::Dup
    )]
    pub student_b: Account<'info, Student>,
    #[account(mut, has_one=dojo)]
    pub board: Account<'info, BoardTape>,
    pub master: Signer<'info>,
}

#[account] pub struct Dojo { pub owner: Pubkey, pub ceiling: u32, pub flow: Drill }
#[account] pub struct Student { pub parent: Pubkey, pub mat: u8, pub exp: u32 }
#[account] pub struct BoardTape { pub parent: Pubkey, pub ring: u8, pub count: u64, pub score: u64 }

#[error_code] pub enum DojoErr { #[msg("duplicate mutable account")] Dup }






