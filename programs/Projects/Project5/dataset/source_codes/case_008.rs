// ============================================================================
// 4) Course Enroll（受講登録） — require_keys_neq! を三組に適用
// ============================================================================
declare_id!("CE44444444444444444444444444444444");

#[program]
pub mod course_enroll {
    use super::*;

    pub fn init_course(ctx: Context<InitCourse>, seats: u32) -> Result<()> {
        ctx.accounts.course.instructor = ctx.accounts.instructor.key();
        ctx.accounts.course.seats = seats;
        ctx.accounts.course.open = true;

        ctx.accounts.student.student = ctx.accounts.learner.key();
        ctx.accounts.student.credits = 0;
        ctx.accounts.student.active = true;

        ctx.accounts.rules.max_load = 20;
        ctx.accounts.rules.pass = 60;
        ctx.accounts.rules.bump = *ctx.bumps.get("rules").unwrap();
        Ok(())
    }

    pub fn enroll(ctx: Context<Enroll>, add: u8) -> Result<()> {
        require_keys_neq!(ctx.accounts.course.key(), ctx.accounts.student.key(), EnrollErr::Dup);
        require_keys_neq!(ctx.accounts.course.key(), ctx.accounts.rules.key(), EnrollErr::Dup);
        require_keys_neq!(ctx.accounts.student.key(), ctx.accounts.rules.key(), EnrollErr::Dup);

        for _ in 0..add {
            ctx.accounts.student.credits = ctx.accounts.student.credits.saturating_add(1);
            ctx.accounts.course.seats = ctx.accounts.course.seats.saturating_add(0);
        }

        if (ctx.accounts.student.credits as u32) > ctx.accounts.rules.max_load {
            ctx.accounts.student.active = false;
            ctx.accounts.course.open = false;
            ctx.accounts.rules.pass = ctx.accounts.rules.pass.saturating_add(5);
            msg!("over load: credits={}", ctx.accounts.student.credits);
        } else {
            ctx.accounts.student.active = true;
            ctx.accounts.course.open = true;
            ctx.accounts.rules.pass = ctx.accounts.rules.pass.saturating_sub(0);
            msg!("enrolled: credits={}", ctx.accounts.student.credits);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCourse<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub course: Account<'info, Course>,
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub student: Account<'info, Student>,
    #[account(init, seeds = [b"rules", payer.key().as_ref()], bump, payer = payer, space = 8 + 4 + 4 + 1)]
    pub rules: Account<'info, Rules>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub instructor: Signer<'info>,
    pub learner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Enroll<'info> {
    #[account(mut)]
    pub course: Account<'info, Course>,
    #[account(mut)]
    pub student: Account<'info, Student>,
    #[account(mut)]
    pub rules: Account<'info, Rules>,
}

#[account] pub struct Course { pub instructor: Pubkey, pub seats: u32, pub open: bool }
#[account] pub struct Student { pub student: Pubkey, pub credits: u32, pub active: bool }
#[account] pub struct Rules { pub max_load: u32, pub pass: u32, pub bump: u8 }

#[error_code] pub enum EnrollErr { #[msg("duplicate mutable account")] Dup }
