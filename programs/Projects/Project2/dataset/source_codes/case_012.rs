// =============================================================================
// 12. Educational Course Platform
// =============================================================================
#[program]
pub mod secure_education {
    use super::*;

    pub fn create_course(ctx: Context<CreateCourse>, title: String, description: String, price: u64) -> Result<()> {
        let course = &mut ctx.accounts.course;
        course.instructor = ctx.accounts.instructor.key();
        course.title = title;
        course.description = description;
        course.price = price;
        course.student_count = 0;
        course.is_published = false;
        course.bump = *ctx.bumps.get("course").unwrap();
        Ok(())
    }

    pub fn publish_course(ctx: Context<PublishCourse>) -> Result<()> {
        let course = &mut ctx.accounts.course;
        course.is_published = true;
        Ok(())
    }

    pub fn enroll_student(ctx: Context<EnrollStudent>) -> Result<()> {
        let course = &mut ctx.accounts.course;
        let enrollment = &mut ctx.accounts.enrollment;
        
        require!(course.is_published, EducationError::CourseNotPublished);
        
        enrollment.course = course.key();
        enrollment.student = ctx.accounts.student.key();
        enrollment.enrolled_at = Clock::get()?.unix_timestamp;
        enrollment.progress = 0;
        enrollment.completed = false;
        enrollment.bump = *ctx.bumps.get("enrollment").unwrap();
        
        course.student_count += 1;
        
        // Payment handling
        **ctx.accounts.student.lamports.borrow_mut() -= course.price;
        **ctx.accounts.instructor.lamports.borrow_mut() += course.price;
        
        Ok(())
    }

    pub fn update_progress(ctx: Context<UpdateProgress>, progress: u8) -> Result<()> {
        let enrollment = &mut ctx.accounts.enrollment;
        
        require!(progress <= 100, EducationError::InvalidProgress);
        
        enrollment.progress = progress;
        if progress == 100 {
            enrollment.completed = true;
        }
        
        Ok(())
    }
}

#[account]
pub struct Course {
    pub instructor: Pubkey,
    pub title: String,
    pub description: String,
    pub price: u64,
    pub student_count: u64,
    pub is_published: bool,
    pub bump: u8,
}

#[account]
pub struct Enrollment {
    pub course: Pubkey,
    pub student: Pubkey,
    pub enrolled_at: i64,
    pub progress: u8,
    pub completed: bool,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateCourse<'info> {
    #[account(
        init,
        payer = instructor,
        space = 8 + 32 + 4 + title.len() + 4 + description.len() + 8 + 8 + 1 + 1,
        seeds = [b"course", instructor.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub course: Account<'info, Course>,
    
    #[account(mut)]
    pub instructor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PublishCourse<'info> {
    #[account(
        mut,
        seeds = [b"course", instructor.key().as_ref(), course.title.as_bytes()],
        bump = course.bump,
        constraint = course.instructor == instructor.key()
    )]
    pub course: Account<'info, Course>,
    
    pub instructor: Signer<'info>,
}

#[derive(Accounts)]
pub struct EnrollStudent<'info> {
    #[account(
        mut,
        seeds = [b"course", course.instructor.as_ref(), course.title.as_bytes()],
        bump = course.bump
    )]
    pub course: Account<'info, Course>,
    
    #[account(
        init,
        payer = student,
        space = 8 + 32 + 32 + 8 + 1 + 1 + 1,
        seeds = [b"enrollment", course.key().as_ref(), student.key().as_ref()],
        bump
    )]
    pub enrollment: Account<'info, Enrollment>,
    
    #[account(mut)]
    pub student: Signer<'info>,
    
    /// CHECK: Verified through course instructor field
    #[account(
        mut,
        constraint = instructor.key() == course.instructor
    )]
    pub instructor: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProgress<'info> {
    #[account(
        mut,
        seeds = [b"enrollment", enrollment.course.as_ref(), student.key().as_ref()],
        bump = enrollment.bump,
        constraint = enrollment.student == student.key()
    )]
    pub enrollment: Account<'info, Enrollment>,
    
    pub student: Signer<'info>,
}

#[error_code]
pub enum EducationError {
    #[msg("Course is not published")]
    CourseNotPublished,
    #[msg("Invalid progress value")]
    InvalidProgress,
}
