// 4. モデレーション構成＋違反ログ
use anchor_lang::prelude::*;
declare_id!("MODR111122223333444455556666777799");

#[program]
pub mod misinit_moderation_v6 {
    use super::*;

    pub fn init_moderation(
        ctx: Context<InitMod>,
        flag_words: Vec<String>,
    ) -> Result<()> {
        require!(flag_words.len() <= 10, ErrorCode4::ManyFlags);
        let m = &mut ctx.accounts.mod_config;
        m.flags = flag_words;
        Ok(())
    }

    pub fn add_flag(
        ctx: Context<InitMod>,
        word: String,
    ) -> Result<()> {
        let m = &mut ctx.accounts.mod_config;
        m.flags.push(word);
        Ok(())
    }

    pub fn record_violation(
        ctx: Context<InitMod>,
        offending: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.violation_log;
        log.offenders.push(offending);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMod<'info> {
    #[account(init, payer = admin, space = 8 + (4+10*32))]
    pub mod_config: Account<'info, ModConfig>,
    #[account(mut)] pub violation_log: Account<'info, ViolationLog>,
    #[account(mut)] pub admin: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct ModConfig { pub flags: Vec<String> }
#[account]
pub struct ViolationLog { pub offenders: Vec<Pubkey> }

#[error_code]
pub enum ErrorCode4 { #[msg("禁止ワードが多すぎます。10 個以内にしてください。")] ManyFlags }

