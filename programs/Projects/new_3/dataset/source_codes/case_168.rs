use anchor_lang::prelude::*;
declare_id!("PharmaVuln11111111111111111111111111111111");

/// 処方箋情報
#[account]
pub struct Prescription {
    pub doctor:      Pubkey, // 医師
    pub patient:     Pubkey, // 患者
    pub fills:       u64,    // 調剤回数
}

/// 調剤記録
#[account]
pub struct FillRecord {
    pub pharmacist:  Pubkey, // 調剤薬剤師
    pub prescription: Pubkey, // 本来は Prescription.key() と一致すべき
    pub times_filled: u64,    // 調剤済み回数
}

#[derive(Accounts)]
pub struct IssuePrescription<'info> {
    #[account(init, payer = doctor, space = 8 + 32 + 32 + 8)]
    pub prescription: Account<'info, Prescription>,
    #[account(mut)]
    pub doctor:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FillMedication<'info> {
    /// Prescription.doctor == doctor.key() は検証される
    #[account(mut, has_one = doctor)]
    pub prescription: Account<'info, Prescription>,

    /// FillRecord.prescription ⇔ prescription.key() の検証がないため、
    /// 任意の FillRecord を渡すと通ってしまう
    #[account(mut)]
    pub fill_record:  Account<'info, FillRecord>,

    pub doctor:       Signer<'info>,
}

#[derive(Accounts)]
pub struct ConfirmFill<'info> {
    /// FillRecord.pharmacist == pharmacist.key() は検証される
    #[account(mut, has_one = pharmacist)]
    pub fill_record:  Account<'info, FillRecord>,

    /// Prescription.key() と fill_record.prescription の一致チェックがない
    #[account(mut)]
    pub prescription: Account<'info, Prescription>,

    pub pharmacist:   Signer<'info>,
}

#[program]
pub mod pharmacy_vuln {
    use super::*;

    /// 新しい処方箋を発行
    pub fn issue_prescription(ctx: Context<IssuePrescription>, patient: Pubkey) -> Result<()> {
        let p = &mut ctx.accounts.prescription;
        p.doctor    = ctx.accounts.doctor.key();
        p.patient   = patient;
        // fills は初期化時点で 0 のまま
        Ok(())
    }

    /// 調剤記録を更新
    pub fn fill_medication(ctx: Context<FillMedication>) -> Result<()> {
        let p  = &mut ctx.accounts.prescription;
        let fr = &mut ctx.accounts.fill_record;

        // 脆弱性ポイント：
        // fr.prescription = p.key(); の代入・検証抜きで実行している
        fr.pharmacist    = ctx.accounts.doctor.key();
        fr.prescription  = p.key();
        fr.times_filled  = fr.times_filled.checked_add(1).unwrap_or_default();

        p.fills = p.fills.checked_add(1).unwrap_or_default();
        Ok(())
    }

    /// 調剤確定（薬剤師のみ実行可能）
    pub fn confirm_fill(ctx: Context<ConfirmFill>) -> Result<()> {
        let p  = &mut ctx.accounts.prescription;
        let fr = &mut ctx.accounts.fill_record;

        // 本来必要：
        // require_keys_eq!(fr.prescription, p.key(), ErrorCode::PrescriptionMismatch);

        p.fills = p.fills.checked_sub(1).unwrap_or_default();
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("FillRecord が指定の Prescription と一致しません")]
    PrescriptionMismatch,
}
