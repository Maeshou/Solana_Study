use anchor_lang::prelude::*;

declare_id!("VulnMulti4444444444444444444444444444444444");

#[program]
pub mod vuln_course {
    use super::*;

    pub fn init_course(
        ctx: Context<InitCourse>,
        capacity: u8,
    ) -> Result<()> {
        let course = &mut ctx.accounts.course;   // ← init OK
        course.capacity = capacity;
        course.enrolled = 0;
        Ok(())
    }

    pub fn setup_roster(
        ctx: Context<SetupRoster>,
        students: Vec<Pubkey>,
    ) -> Result<()> {
        let course = &mut ctx.accounts.course;
        let roster = &mut ctx.accounts.roster;   // ← init missing
        roster.students = Vec::new();
        for pk in students.iter().take(course.capacity as usize) {
            roster.students.push(*pk);
            course.enrolled += 1;
        }
        Ok(())
    }

    pub fn record_stats(ctx: Context<RecordStats>) -> Result<()> {
        let course = &ctx.accounts.course;
        let stats  = &mut ctx.accounts.stats;     // ← init OK
        stats.enrolled   = course.enrolled as u32;
        stats.started_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCourse<'info> {
    #[account(init, payer = teacher, space = 8 + 1 + 1)]
    pub course: Account<'info, CourseData>,
    pub roster: Account<'info, RosterData>,     // ← init missing
    #[account(init, payer = teacher, space = 8 + 4 + 8)]
    pub stats: Account<'info, StatsData>,
    #[account(mut)] pub teacher: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetupRoster<'info> {
    #[account(mut)] pub course: Account<'info, CourseData>,
    pub roster: Account<'info, RosterData>,     // ← init missing
}

#[derive(Accounts)]
pub struct RecordStats<'info> {
    pub course: Account<'info, CourseData>,
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
