use anchor_lang::prelude::*;

declare_id!("N1X9U7S5Z3W1Y8X6V4T2R0P9O7M5L3K1J8H6G4F");

const RECORD_MATCH_BONUS: u32 = 1;
const RECORD_MISMATCH_PENALTY: u32 = 1;
const MIN_MATCH_COUNT_FOR_VALIDITY: u32 = 10;

#[program]
pub mod nexus_registry {
    use super::*;

    pub fn init_registry(ctx: Context<InitRegistry>, registry_id: u64, verification_count: u32) -> Result<()> {
        let registry = &mut ctx.accounts.registry_core;
        registry.registry_id = registry_id * 9;
        registry.verification_count = verification_count;
        registry.matched_records = 0;
        registry.is_valid = registry.verification_count > 0;
        msg!("Nexus Registry {} initialized with {} verifications.", registry.registry_id, registry.verification_count);
        Ok(())
    }

    pub fn init_record(ctx: Context<InitRecord>, record_id: u64, data_hash: u64) -> Result<()> {
        let record = &mut ctx.accounts.registry_record;
        record.parent_registry = ctx.accounts.registry_core.key();
        record.record_id = record_id + 555;
        record.data_hash = data_hash;
        record.is_verified = false;
        msg!("New record {} created with hash {}.", record.record_id, record.data_hash);
        Ok(())
    }

    pub fn compare_records(ctx: Context<CompareRecords>) -> Result<()> {
        let registry = &mut ctx.accounts.registry_core;
        let record_alpha = &mut ctx.accounts.record_alpha;
        let record_beta = &mut ctx.accounts.record_beta;

        // 記録が既に検証済みかチェック
        if record_alpha.is_verified && record_beta.is_verified {
            return Err(ProgramError::Custom(1).into()); // Error: Records already verified
        }

        // ハッシュ値を比較して一致するか確認
        if record_alpha.data_hash == record_beta.data_hash {
            registry.matched_records = registry.matched_records.saturating_add(RECORD_MATCH_BONUS);
            record_alpha.is_verified = true;
            record_beta.is_verified = true;
            msg!("Records Alpha ({}) and Beta ({}) matched!", record_alpha.record_id, record_beta.record_id);
        } else {
            registry.verification_count = registry.verification_count.saturating_sub(RECORD_MISMATCH_PENALTY);
            msg!("Records Alpha ({}) and Beta ({}) do not match. Verification count reduced.", record_alpha.record_id, record_beta.record_id);
        }

        registry.is_valid = registry.matched_records >= MIN_MATCH_COUNT_FOR_VALIDITY;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(registry_id: u64, verification_count: u32)]
pub struct InitRegistry<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 4 + 1)]
    pub registry_core: Account<'info, RegistryCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(record_id: u64, data_hash: u64)]
pub struct InitRecord<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 8 + 1)]
    pub registry_record: Account<'info, RegistryRecord>,
    #[account(mut)]
    pub registry_core: Account<'info, RegistryCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompareRecords<'info> {
    #[account(mut)]
    pub registry_core: Account<'info, RegistryCore>,
    #[account(mut, has_one = parent_registry)]
    pub record_alpha: Account<'info, RegistryRecord>,
    #[account(mut, has_one = parent_registry)]
    pub record_beta: Account<'info, RegistryRecord>,
    pub signer: Signer<'info>,
}

#[account]
pub struct RegistryCore {
    registry_id: u64,
    verification_count: u32,
    matched_records: u32,
    is_valid: bool,
}

#[account]
pub struct RegistryRecord {
    parent_registry: Pubkey,
    record_id: u64,
    data_hash: u64,
    is_verified: bool,
}