// 1) SoftCapBurstRouter: 残余キャパがしきい値以下なら外部プログラムへ
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("SoftCapBurst11111111111111111111111111111");

#[program]
pub mod soft_cap_burst_router {
    use super::*;
    pub fn initialize(ctx: Context<InitSoftCap>, unit_tokens: u64, hard_cap: u64, near_cap_margin: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.operator = ctx.accounts.operator.key();
        config.unit_tokens = unit_tokens.max(1);
        config.hard_cap = hard_cap.max(config.unit_tokens);
        config.near_cap_margin = near_cap_margin;
        config.total_sent = 0;
        Ok(())
    }
    pub fn process(ctx: Context<ProcSoftCap>, rounds: u8) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let mut loop_index: u8 = 0;

        while loop_index < rounds {
            let projected = config.total_sent.saturating_add(config.unit_tokens);
            if projected > config.hard_cap { return Err(SoftCapErr::Cap.into()); }

            let remaining_capacity = config.hard_cap.saturating_sub(config.total_sent);
            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            if remaining_capacity <= config.near_cap_margin { program_account_info = ctx.accounts.external_program.clone(); }

            token::approve(ctx.accounts.approve_with(program_account_info.clone()), config.unit_tokens)?;
            token::transfer(ctx.accounts.transfer_with(program_account_info.clone()), config.unit_tokens)?;
            token::revoke(ctx.accounts.revoke_with(program_account_info))?;

            config.total_sent = projected;
            loop_index = loop_index.saturating_add(1);
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitSoftCap<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub config: Account<'info, SoftCapState>,
    #[account(mut)] pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ProcSoftCap<'info> {
    #[account(mut, has_one = operator)]
    pub config: Account<'info, SoftCapState>,
    pub operator: Signer<'info>,
    #[account(mut)] pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)] pub target_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}
impl<'info> ProcSoftCap<'info> {
    fn approve_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(p, Approve { to: self.source_vault.to_account_info(), delegate: self.target_vault.to_account_info(), authority: self.operator.to_account_info() })
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(p, Transfer { from: self.source_vault.to_account_info(), to: self.target_vault.to_account_info(), authority: self.operator.to_account_info() })
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(p, Revoke { source: self.source_vault.to_account_info(), authority: self.operator.to_account_info() })
    }
}
#[account] pub struct SoftCapState { pub operator: Pubkey, pub unit_tokens: u64, pub hard_cap: u64, pub near_cap_margin: u64, pub total_sent: u64 }
#[error_code] pub enum SoftCapErr { #[msg("cap reached")] Cap }
