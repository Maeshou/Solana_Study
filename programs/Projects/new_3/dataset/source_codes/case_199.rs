use anchor_lang::prelude::*;
declare_id!("TimesheetVuln11111111111111111111111111111");

/// 勤怠提出情報
#[account]
pub struct Timesheet {
    pub employee:    Pubkey,        // 社員
    pub period:      String,        // 対象期間
    pub submitted:   bool,          // 提出済みフラグ
}

/// 承認記録
#[account]
pub struct ApprovalRecord {
    pub approver:    Pubkey,        // 承認者
    pub timesheet:   Pubkey,        // 本来は Timesheet.key() と一致すべき
    pub comment:     String,        // コメント
}

#[derive(Accounts)]
pub struct SubmitSheet<'info> {
    #[account(init, payer = employee, space = 8 + 32 + 4 + 32 + 1)]
    pub timesheet:   Account<'info, Timesheet>,
    #[account(mut)]
    pub employee:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateApproval<'info> {
    /// Timesheet.employee == employee.key() は不要だが例示
    #[account(mut, has_one = employee)]
    pub timesheet:   Account<'info, Timesheet>,

    /// ApprovalRecord.timesheet ⇔ timesheet.key() の検証がないため、
    /// 偽のレコードで任意の勤怠を承認できる
    #[account(init, payer = approver, space = 8 + 32 + 32 + 4 + 128)]
    pub approval:    Account<'info, ApprovalRecord>,

    #[account(mut)]
    pub employee:    Signer<'info>,
    #[account(mut)]
    pub approver:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeApproval<'info> {
    /// ApprovalRecord.approver == approver.key() は検証される
    #[account(mut, has_one = approver)]
    pub approval:    Account<'info, ApprovalRecord>,

    /// timesheet.key() ⇔ approval.timesheet の検証がないため、
    /// 偽物のレコードで別の勤怠を最終承認できる
    #[account(mut)]
    pub timesheet:   Account<'info, Timesheet>,

    pub approver:    Signer<'info>,
}

#[program]
pub mod timesheet_vuln {
    use super::*;

    /// 勤怠を提出
    pub fn submit_sheet(ctx: Context<SubmitSheet>, period: String) -> Result<()> {
        let ts = &mut ctx.accounts.timesheet;
        ts.employee  = ctx.accounts.employee.key();
        ts.period    = period;
        ts.submitted = true;
        Ok(())
    }

    /// 承認を作成（コメント記録）
    pub fn create_approval(ctx: Context<CreateApproval>, comment: String) -> Result<()> {
        let ap = &mut ctx.accounts.approval;
        // 脆弱性ポイント:
        // ap.timesheet = ctx.accounts.timesheet.key(); の照合制約がない
        ap.approver  = ctx.accounts.approver.key();
        ap.timesheet = ctx.accounts.timesheet.key();
        ap.comment   = comment;
        Ok(())
    }

    /// 承認を最終確定（勤怠提出に反映）
    pub fn finalize_approval(ctx: Context<FinalizeApproval>) -> Result<()> {
        let ts = &mut ctx.accounts.timesheet;
        // 本来必要:
        // require_keys_eq!(ctx.accounts.approval.timesheet, ts.key(), ErrorCode::Mismatch);

        // submitted フラグを false にして最終承認完了扱い
        ts.submitted = false;
        Ok(())
    }
}
