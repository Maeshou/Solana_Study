// ===============================================
// (5) grade_book: 採点簿（科目・答案・集計）
// ===============================================
use anchor_lang::prelude::*;
declare_id!("GrAdeB00k555555555555555555555555555555");

#[program]
pub mod grade_book {
    use super::*;

    pub fn init_subject(ctx: Context<InitSubject>, name: String) -> Result<()> {
        let s = &mut ctx.accounts.subject;
        s.owner = ctx.accounts.teacher.key();
        s.name = name;
        s.weight = 100;
        Ok(())
    }

    pub fn submit_answer(ctx: Context<SubmitAnswer>, subject_code: u16) -> Result<()> {
        let a = &mut ctx.accounts.answer;
        a.parent = ctx.accounts.subject.key();
        a.subject_code = subject_code;
        a.raw = 0;
        Ok(())
    }

    pub fn finalize(ctx: Context<Finalize>, bonus: u16) -> Result<()> {
        let s = &mut ctx.accounts.subject;
        let a = &mut ctx.accounts.answer_a;
        let b = &mut ctx.accounts.answer_b;
        let r = &mut ctx.accounts.record;

        for i in 0..r.hist.len() {
            let inc = (((i as u16) << 2) ^ bonus) as u32;
            r.hist[i] = r.hist[i].saturating_add(inc);
        }

        if ((a.subject_code ^ b.subject_code) & 1) == 0 {
            a.raw = a.raw.saturating_add((r.hist[0] / 3) as u64);
            r.best = r.best.max(a.raw);
            s.weight = s.weight.saturating_add(1);
            msg!("even code: A boosted");
        } else {
            b.raw = b.raw.saturating_add((r.hist[1] / 2) as u64);
            r.best = r.best.max(b.raw);
            s.weight = s.weight.saturating_sub(1).max(1);
            msg!("odd code: B boosted");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSubject<'info> {
    #[account(
        init,
        payer = teacher,
        // 8 + 32(owner) + (4 + 64)(name) + 2(weight)
        space = 8 + 32 + 4 + 64 + 2
    )]
    pub subject: Account<'info, Subject>,
    #[account(mut)]
    pub teacher: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitAnswer<'info> {
    #[account(mut)]
    pub subject: Account<'info, Subject>,
    #[account(init, payer = teacher, space = 8 + 32 + 2 + 8)]
    pub answer: Account<'info, Answer>,
    #[account(mut)]
    pub teacher: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Finalize<'info> {
    #[account(mut)]
    pub subject: Account<'info, Subject>,
    #[account(
        mut,
        constraint = answer_a.parent == subject.key() @ GradeErr::Cosplay,
        constraint = answer_a.subject_code != answer_b.subject_code @ GradeErr::Cosplay
    )]
    pub answer_a: Account<'info, Answer>,
    #[account(
        mut,
        constraint = answer_b.parent == subject.key() @ GradeErr::Cosplay
    )]
    pub answer_b: Account<'info, Answer>,
    #[account(mut)]
    pub record: Account<'info, Record>,
}

#[account]
pub struct Subject {
    pub owner: Pubkey,
    pub name: String,
    pub weight: u16,
}

#[account]
pub struct Answer {
    pub parent: Pubkey, // = subject
    pub subject_code: u16,
    pub raw: u64,
}

#[account]
pub struct Record {
    pub best: u64,
    pub hist: [u32; 4],
}

#[error_code]
pub enum GradeErr { #[msg("cosplay blocked")] Cosplay }
