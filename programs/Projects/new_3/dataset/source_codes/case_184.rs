use anchor_lang::prelude::*;
declare_id!("EduVulnNoMatch11111111111111111111111111");

/// 講座情報
#[account]
pub struct Course {
    pub owner:       Pubkey,        // 講師（主催者）
    pub title:       String,        // 講座名
    pub students:    Vec<Pubkey>,   // 登録学生一覧
}

/// 受講登録情報
#[account]
pub struct Enrollment {
    pub student:     Pubkey,        // 学生
    pub course:      Pubkey,        // 本来は Course.key() と一致すべき
    pub status:      String,        // "enrolled" または "completed"
}

#[derive(Accounts)]
pub struct CreateCourse<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 4 + (32 * 50))]
    pub course:      Account<'info, Course>,
    #[account(mut)]
    pub owner:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Enroll<'info> {
    /// Course.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub course:      Account<'info, Course>,

    /// Enrollment.course ⇔ course.key() の検証がない
    #[account(init, payer = student, space = 8 + 32 + 32 + 4 + 16)]
    pub enrollment:  Account<'info, Enrollment>,

    #[account(mut)]
    pub owner:       Signer<'info>,
    #[account(mut)]
    pub student:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteCourse<'info> {
    /// Enrollment.student == student.key() は検証される
    #[account(mut, has_one = student)]
    pub enrollment:  Account<'info, Enrollment>,

    /// course.key() ⇔ enrollment.course の検証がない
    #[account(mut)]
    pub course:      Account<'info, Course>,

    pub student:     Signer<'info>,
}

#[program]
pub mod education_vuln {
    use super::*;

    /// 新規講座を作成
    pub fn create_course(ctx: Context<CreateCourse>, title: String) -> Result<()> {
        let c = &mut ctx.accounts.course;
        c.owner    = ctx.accounts.owner.key();
        c.title    = title;
        // students は init 時に空ベクタになる
        Ok(())
    }

    /// 学生を講座に登録
    pub fn enroll(ctx: Context<Enroll>) -> Result<()> {
        let c  = &mut ctx.accounts.course;
        let e  = &mut ctx.accounts.enrollment;

        // 脆弱性ポイント:
        // e.course = c.key(); の検証・制約がない
        e.student = ctx.accounts.student.key();
        e.course  = c.key();
        e.status  = String::from("enrolled");

        // Vec::push で登録学生一覧に追加
        c.students.push(e.student);
        Ok(())
    }

    /// 受講完了をマーク
    pub fn complete_course(ctx: Context<CompleteCourse>) -> Result<()> {
        let c  = &mut ctx.accounts.course;
        let e  = &mut ctx.accounts.enrollment;

        // 本来必要:
        // require_keys_eq!(e.course, c.key(), ErrorCode::CourseMismatch);

        // ステータス変更と講座タイトル末尾にタグ付け
        e.status = String::from("completed");
        c.title  = format!("{} [Completed by {}]", c.title, e.student);
        Ok(())
    }
}
