// 10) approval_console: 承認／却下／保留（短い関数はここに集約）
use anchor_lang::prelude::*;

declare_id!("Appr0v4lAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod approval_console {
    use super::*;

    pub fn approve(ctx: Context<Moderate>) -> Result<()> {
        ctx.accounts.app.status = RegistrationStatus::Approved;
        ctx.accounts.app.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
    pub fn reject(ctx: Context<Moderate>) -> Result<()> {
        ctx.accounts.app.status = RegistrationStatus::Rejected;
        ctx.accounts.app.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
    pub fn reset(ctx: Context<Moderate>) -> Result<()> {
        ctx.accounts.app.status = RegistrationStatus::Pending;
        ctx.accounts.app.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Moderate<'info> {
    #[account(
        init_if_needed,
        payer = moderator,
        space = 8 + ApprovalState::LEN,
        seeds = [b"approval", moderator.key().as_ref()],
        bump
    )]
    pub app: Account<'info, ApprovalState>,
    #[account(mut)]
    pub moderator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ApprovalState {
    pub status: RegistrationStatus,
    pub updated_at: i64,
}
impl ApprovalState { pub const LEN: usize = 1 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum RegistrationStatus { Pending, Approved, Rejected }
