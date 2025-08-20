use anchor_lang::prelude::*;

declare_id!("Ex6000000000000000000000000000000000006");

#[program]
pub mod example6 {
    use super::*;

    // 講座を作成し、空席フラグを設定
    pub fn create_course(ctx: Context<CreateCourse>, capacity: u8) -> Result<()> {
        let c = &mut ctx.accounts.course;           // ← initあり
        c.capacity = capacity;
        c.filled = 0;
        c.has_space = capacity > 0;
        Ok(())
    }

    // 受講者を登録し、空席更新・最終受講時刻を記録
    pub fn enroll(ctx: Context<Enroll>, want: u8) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let c = &mut ctx.accounts.course;           // ← initなし：既存参照のみ
        let mut i = 0u8;
        while i < want {
            if c.filled >= c.capacity {
                break;
            }
            c.filled += 1;
            i += 1;
        }
        c.has_space = c.filled < c.capacity;
        c.last_enroll = now;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCourse<'info> {
    #[account(init, payer = teacher, space = 8 + 1*4 + 8)]
    pub course: Account<'info, CourseData>,
    #[account(mut)] pub teacher: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Enroll<'info> {
    pub course: Account<'info, CourseData>,
    #[account(mut)] pub teacher: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CourseData {
    pub capacity: u8,
    pub filled: u8,
    pub has_space: bool,
    pub last_enroll: i64,
}
