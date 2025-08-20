use anchor_lang::prelude::*;
declare_id!("MedSchedVuln111111111111111111111111111111");

/// 医師の診療スケジュール
#[account]
pub struct Schedule {
    pub doctor:      Pubkey, // スケジュール管理者（医師）
    pub total_slots: u64,    // 総診療枠数
}

/// 患者の予約記録
#[account]
pub struct Appointment {
    pub patient:     Pubkey, // 予約患者
    pub schedule:    Pubkey, // 本来は Schedule.key() と一致すべき
    pub time_slot:   i64,    // 予約時間（UNIXタイム）
}

#[derive(Accounts)]
pub struct CreateSchedule<'info> {
    #[account(init, payer = doctor, space = 8 + 32 + 8)]
    pub schedule:    Account<'info, Schedule>,
    #[account(mut)]
    pub doctor:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BookAppointment<'info> {
    /// Schedule.doctor == doctor.key() は検証される
    #[account(mut, has_one = doctor)]
    pub schedule:    Account<'info, Schedule>,

    /// Appointment.schedule ⇔ schedule.key() の検証がないため、
    /// 任意の Appointment アカウントを渡しても通ってしまう
    #[account(init, payer = patient, space = 8 + 32 + 32 + 8)]
    pub appointment: Account<'info, Appointment>,

    #[account(mut)]
    pub doctor:      Signer<'info>,
    #[account(mut)]
    pub patient:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmAppointment<'info> {
    /// Appointment.patient == patient.key() は検証される
    #[account(mut, has_one = patient)]
    pub appointment: Account<'info, Appointment>,

    /// schedule.key() と appointment.schedule の照合がない
    #[account(mut)]
    pub schedule:    Account<'info, Schedule>,

    pub patient:     Signer<'info>,
}

#[program]
pub mod medical_schedule_vuln {
    use super::*;

    /// 診療スケジュールを作成
    pub fn create_schedule(ctx: Context<CreateSchedule>, slots: u64) -> Result<()> {
        let s = &mut ctx.accounts.schedule;
        s.doctor      = ctx.accounts.doctor.key();
        s.total_slots = slots;
        Ok(())
    }

    /// 診療予約を行う
    pub fn book_appointment(ctx: Context<BookAppointment>, time_slot: i64) -> Result<()> {
        let s = &mut ctx.accounts.schedule;
        let a = &mut ctx.accounts.appointment;

        // 脆弱性ポイント:
        // a.schedule = s.key(); と設定するのみで、
        // Appointment.schedule と Schedule.key() の一致検証を行っていない
        a.patient   = ctx.accounts.patient.key();
        a.schedule  = s.key();
        a.time_slot = time_slot;

        // 予約枠消費を示すため total_slots をデクリメント
        s.total_slots = s.total_slots.checked_sub(1).unwrap_or(s.total_slots);
        Ok(())
    }

    /// 患者が予約を確定
    pub fn confirm_appointment(ctx: Context<ConfirmAppointment>) -> Result<()> {
        let s = &mut ctx.accounts.schedule;
        // 本来は必須:
        // require_keys_eq!(ctx.accounts.appointment.schedule, s.key(), ErrorCode::ScheduleMismatch);

        // 確定に伴い残枠をさらに減らす
        s.total_slots = s.total_slots.checked_sub(1).unwrap_or(s.total_slots);
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Appointment が指定の Schedule と一致しません")]
    ScheduleMismatch,
}
