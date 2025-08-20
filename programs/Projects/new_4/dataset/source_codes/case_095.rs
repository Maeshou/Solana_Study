use anchor_lang::prelude::*;

declare_id!("VulnInit4444444444444444444444444444444444");

#[program]
pub mod vuln_course {
    use super::*;

    pub fn init_course(
        ctx: Context<InitCourse>,
        capacity: u8,
        default_students: Vec<Pubkey>,
    ) -> Result<()> {
        let course = &mut ctx.accounts.course;       // ← Init OK
        course.capacity = capacity;
        course.enrolled = 0;

        // roster は init されていない → 任意アドレス差し替え可
        let roster = &mut ctx.accounts.roster;       // ← Init missing
        roster.students = Vec::new();
        for pk in default_students.iter().take(capacity as usize) {
            roster.students.push(*pk);
            course.enrolled += 1;
        }

        let stats = &mut ctx.accounts.stats;         // ← Init OK
        stats.enrolled   = course.enrolled as u32;
        stats.started_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCourse<'info> {
    #[account(init, payer = teacher, space = 8 + 1 + 1)]
    pub course: Account<'info, CourseData>,
    pub roster: Account<'info, RosterData>,        // ← init がない
    #[account(init, payer = teacher, space = 8 + 4 + 8)]
    pub stats: Account<'info, StatsData>,
    #[account(mut)] pub teacher: Signer<'info>,
    pub system_program: Program<'info, System>,
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
