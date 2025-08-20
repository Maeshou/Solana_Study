use anchor_lang::prelude::*;

declare_id!("SafeEx03XXXXXXX3333333333333333333333333333");

#[program]
pub mod example3 {
    use super::*;

    pub fn init_course(
        ctx: Context<InitCourse>,
        capacity: u8,
        initial: u8,
    ) -> Result<()> {
        let course = &mut ctx.accounts.course;
        course.capacity   = capacity;
        course.registered = 0;

        // 初期登録者IDをxor累積
        let mut xor_id = 0u8;
        for i in 1..=initial.min(capacity) {
            course.registered += 1;
            xor_id ^= i;
        }

        let counter = &mut ctx.accounts.counter;
        counter.count  = course.registered;
        counter.xor_id = xor_id;

        let stats = &mut ctx.accounts.stats;
        stats.full     = course.registered == capacity;
        stats.enrolled = course.registered as u32;
        Ok(())
    }

    pub fn register(
        ctx: Context<Register>,
    ) -> Result<()> {
        let course = &mut ctx.accounts.course;
        let counter= &mut ctx.accounts.counter;
        let stats  = &mut ctx.accounts.stats;

        // 二段階チェック
        if course.registered < course.capacity {
            course.registered += 1;
            counter.count += 1;
            // カウントが偶数なら +10 ボーナス
            if counter.count % 2 == 0 {
                counter.count += 10;
            }
        }
        stats.full     = course.registered == course.capacity;
        stats.enrolled = course.registered as u32;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCourse<'info> {
    #[account(init, payer = teacher, space = 8 + 1 + 1)]
    pub course: Account<'info, CourseData>,
    #[account(init, payer = teacher, space = 8 + 1 + 1)]
    pub counter: Account<'info, CounterData>,
    #[account(init, payer = teacher, space = 8 + 1 + 4)]
    pub stats: Account<'info, StatsData>,
    #[account(mut)] pub teacher: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)] pub course: Account<'info, CourseData>,
    #[account(mut)] pub counter: Account<'info, CounterData>,
    #[account(mut)] pub stats: Account<'info, StatsData>,
}

#[account]
pub struct CourseData {
    pub capacity: u8,
    pub registered: u8,
}

#[account]
pub struct CounterData {
    pub count: u8,
    pub xor_id: u8,
}

#[account]
pub struct StatsData {
    pub full: bool,
    pub enrolled: u32,
}
