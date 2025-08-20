// =============================================================================
// 18. Healthcare Records Management
// =============================================================================
#[program]
pub mod secure_healthcare {
    use super::*;

    pub fn create_patient_record(ctx: Context<CreatePatientRecord>, name: String, date_of_birth: i64) -> Result<()> {
        let patient_record = &mut ctx.accounts.patient_record;
        patient_record.patient = ctx.accounts.patient.key();
        patient_record.name = name;
        patient_record.date_of_birth = date_of_birth;
        patient_record.created_at = Clock::get()?.unix_timestamp;
        patient_record.authorized_doctors = Vec::new();
        patient_record.bump = *ctx.bumps.get("patient_record").unwrap();
        Ok(())
    }

    pub fn authorize_doctor(ctx: Context<AuthorizeDoctor>) -> Result<()> {
        let patient_record = &mut ctx.accounts.patient_record;
        
        require!(!patient_record.authorized_doctors.contains(ctx.accounts.doctor.key), HealthcareError::DoctorAlreadyAuthorized);
        
        patient_record.authorized_doctors.push(*ctx.accounts.doctor.key);
        Ok(())
    }

    pub fn add_medical_record(ctx: Context<AddMedicalRecord>, diagnosis: String, treatment: String, notes: String) -> Result<()> {
        let patient_record = &ctx.accounts.patient_record;
        let medical_record = &mut ctx.accounts.medical_record;
        
        require!(patient_record.authorized_doctors.contains(&ctx.accounts.doctor.key()), HealthcareError::DoctorNotAuthorized);
        
        medical_record.patient_record = patient_record.key();
        medical_record.doctor = ctx.accounts.doctor.key();
        medical_record.diagnosis = diagnosis;
        medical_record.treatment = treatment;
        medical_record.notes = notes;
        medical_record.created_at = Clock::get()?.unix_timestamp;
        medical_record.bump = *ctx.bumps.get("medical_record").unwrap();
        
        Ok(())
    }

    pub fn revoke_doctor_access(ctx: Context<RevokeDoctorAccess>) -> Result<()> {
        let patient_record = &mut ctx.accounts.patient_record;
        
        patient_record.authorized_doctors.retain(|&x| x != *ctx.accounts.doctor.key);
        Ok(())
    }
}

#[account]
pub struct PatientRecord {
    pub patient: Pubkey,
    pub name: String,
    pub date_of_birth: i64,
    pub created_at: i64,
    pub authorized_doctors: Vec<Pubkey>,
    pub bump: u8,
}

#[account]
pub struct MedicalRecord {
    pub patient_record: Pubkey,
    pub doctor: Pubkey,
    pub diagnosis: String,
    pub treatment: String,
    pub notes: String,
    pub created_at: i64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreatePatientRecord<'info> {
    #[account(
        init,
        payer = patient,
        space = 8 + 32 + 4 + name.len() + 8 + 8 + 4 + (32 * 10) + 1, // Allow up to 10 authorized doctors
        seeds = [b"patient", patient.key().as_ref()],
        bump
    )]
    pub patient_record: Account<'info, PatientRecord>,
    
    #[account(mut)]
    pub patient: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AuthorizeDoctor<'info> {
    #[account(
        mut,
        seeds = [b"patient", patient.key().as_ref()],
        bump = patient_record.bump,
        constraint = patient_record.patient == patient.key()
    )]
    pub patient_record: Account<'info, PatientRecord>,
    
    pub patient: Signer<'info>,
    
    /// CHECK: Doctor account to be authorized
    pub doctor: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(diagnosis: String, treatment: String, notes: String)]
pub struct AddMedicalRecord<'info> {
    #[account(
        seeds = [b"patient", patient_record.patient.as_ref()],
        bump = patient_record.bump
    )]
    pub patient_record: Account<'info, PatientRecord>,
    
    #[account(
        init,
        payer = doctor,
        space = 8 + 32 + 32 + 4 + diagnosis.len() + 4 + treatment.len() + 4 + notes.len() + 8 + 1,
        seeds = [b"medical_record", patient_record.key().as_ref(), &Clock::get().unwrap().unix_timestamp.to_le_bytes()],
        bump
    )]
    pub medical_record: Account<'info, MedicalRecord>,
    
    #[account(mut)]
    pub doctor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeDoctorAccess<'info> {
    #[account(
        mut,
        seeds = [b"patient", patient.key().as_ref()],
        bump = patient_record.bump,
        constraint = patient_record.patient == patient.key()
    )]
    pub patient_record: Account<'info, PatientRecord>,
    
    pub patient: Signer<'info>,
    
    /// CHECK: Doctor account to revoke access from
    pub doctor: AccountInfo<'info>,
}

#[error_code]
pub enum HealthcareError {
    #[msg("Doctor is already authorized")]
    DoctorAlreadyAuthorized,
    #[msg("Doctor is not authorized to access this record")]
    DoctorNotAuthorized,
}
