use anchor_lang::prelude::*;

declare_id!("SafeMulti4444444444444444444444444444444444");

#[program]
pub mod safe_course {
    use super::*;

    // course, roster, stats をすべて初期化
    pub fn init_course(
        ctx: Context<InitCourse>,
        capacity: u8,
        default_students: Vec<Pubkey>,
    ) -> Result<()> {
        let course = &mut ctx.accounts.course;
        course.capacity = capacity;
        course.enrolled = 0;

        let roster = &mut ctx.accounts.roster;
        roster.students = Vec::new();
        for pk in default_students.iter().take(capacity as usize) {
            roster.students.push(*pk);
            course.enrolled += 1;
        }

        let stats = &mut ctx.accounts.stats;
        stats.enrolled = course.enrolled as u32;
        stats.started_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    // roster, course, stats を mut 更新
    pub fn add_student(ctx: Context<AddStudent>, student: Pubkey) -> Result<()> {
        let course = &mut ctx.accounts.course;
        let roster = &mut ctx.accounts.roster;
        let stats  = &mut ctx.accounts.stats;

        if course.enrolled < course.capacity {
            roster.students.push(student);
            course.enrolled += 1;
            stats.enrolled = course.enrolled as u32;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCourse<'info> {
    #[account(init, payer = teacher, space = 8 + 1 + 1)]
    pub course: Account<'info, CourseData>,
    #[account(init, payer = teacher, space = 8 + 4 + (32 * 10))]
    pub roster: Account<'info, RosterData>,
    #[account(init, payer = teacher, space = 8 + 4 + 8)]
    pub stats: Account<'info, StatsData>,
    #[account(mut)] pub teacher: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddStudent<'info> {
    #[account(mut)] pub course: Account<'info, CourseData>,
    #[account(mut)] pub roster: Account<'info, RosterData>,
    #[account(mut)] pub stats: Account<'info, StatsData>,
}

#[account]
pub struct CourseData {
    pub capacity: u8,
    pub enrolled: u8,
}

#[account]
pub struct RosterData {
    pub students: Vec<Pubkey>,
}

#[account]
pub struct StatsData {
    pub enrolled: u32,
    pub started_at: i64,
}
