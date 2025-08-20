// 3. コンテンツモデレーション＋違反履歴
use anchor_lang::prelude::*;
declare_id!("MODV111122223333444455556666777788");

#[program]
pub mod misinit_moderation_v7 {
    use super::*;

    pub fn init_moderation(
        ctx: Context<InitMod>,
        threshold: u8,
    ) -> Result<()> {
        require!(threshold <= 100, ErrorCode3::ThresholdTooHigh);
        let m = &mut ctx.accounts.mod_config;
        m.threshold = threshold;
        Ok(())
    }

    pub fn flag_content(
        ctx: Context<InitMod>,
        content_id: u64,
    ) -> Result<()> {
        let m = &mut ctx.accounts.mod_config;
        m.flags.push(content_id);
        Ok(())
    }

    pub fn log_infraction(
        ctx: Context<InitMod>,
        offender: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.infraction_log;
        log.offenders.push(offender);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMod<'info> {
    #[account(init, payer = moderator, space = 8 + 1 + (4+16))]
    pub mod_config: Account<'info, ModConfig>,
    #[account(mut)] pub infraction_log: Account<'info, InfractionLog>,
    #[account(mut)] pub moderator: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct ModConfig { pub threshold: u8, pub flags: Vec<u64> }
#[account]
pub struct InfractionLog { pub offenders: Vec<Pubkey> }

#[error_code]
pub enum ErrorCode3 { #[msg("閾値が高すぎます。0-100の範囲で指定してください。")] ThresholdTooHigh }
