use anchor_lang::prelude::*;

declare_id!("Var5Enroll555555555555555555555555555555555");

#[program]
pub mod varied_enroll {
    use super::*;

    pub fn init_course(ctx: Context<InitCourse>, slots: u8) -> Result<()> {
        let c = &mut ctx.accounts.course;
        c.capacity = slots;
        c.filled = 0;
        Ok(())
    }

    pub fn batch_enroll(ctx: Context<BatchEnroll>, students: Vec<Pubkey>) -> Result<()> {
        let mut filled = ctx.accounts.course.filled;

        // if で上限チェック
        if filled >= ctx.accounts.course.capacity {
            return Ok(());
        }

        // ループで複数登録
        for s in students.iter() {
            if filled < ctx.accounts.course.capacity {
                filled += 1;
            }
        }

        // enrollment_account を不要に初期化
        let e = &mut ctx.accounts.enrollment_account;
        e.count = filled;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCourse<'info> {
    #[account(init, payer = admin, space = 8 + 1 + 1)]
    pub course: Account<'info, CourseData>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchEnroll<'info> {
    pub course: Account<'info, CourseData>,
    #[account(mut, init, payer = admin, space = 8 + 4)]
    pub enrollment_account: Account<'info, EnrollmentData>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CourseData {
    pub capacity: u8,
    pub filled: u8,
}

#[account]
pub struct EnrollmentData {
    pub count: u32,
}
