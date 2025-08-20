// (6) ProgramFromDataKey: 保存した Pubkey に一致する AccountInfo を使って approve/transfer/revoke
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Revoke, Transfer, Token, TokenAccount};

declare_id!("ProgFromKey66666666666666666666666666666");

#[program]
pub mod program_from_data_key {
    use super::*;
    pub fn setup_options(ctx: Context<SetupOptions>, allowed_program: Pubkey) -> Result<()> {
        let options_state = &mut ctx.accounts.options_state;
        options_state.signer = ctx.accounts.signer.key();
        options_state.allowed_program = allowed_program;
        options_state.total_processed = 0;
        Ok(())
    }

    pub fn process(ctx: Context<Process>, move_amount: u64) -> Result<()> {
        let options_state = &mut ctx.accounts.options_state;
        let mut program_account: Option<AccountInfo> = None;

        for account_info_entry in ctx.remaining_accounts.iter() {
            if account_info_entry.key() == options_state.allowed_program {
                program_account = Some(account_info_entry.clone());
                break;
            }
        }
        let program_account = program_account.ok_or(ProcessErr::ProgramMissing)?;

        token::approve(CpiContext::new(program_account.clone(), Approve {
            to: ctx.accounts.source_tokens.to_account_info(),
            delegate: ctx.accounts.destination_tokens.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        }), move_amount)?;

        token::transfer(CpiContext::new(program_account.clone(), Transfer {
            from: ctx.accounts.source_tokens.to_account_info(),
            to: ctx.accounts.destination_tokens.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        }), move_amount)?;

        token::revoke(CpiContext::new(program_account, Revoke {
            source: ctx.accounts.source_tokens.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        }))?;

        options_state.total_processed = options_state.total_processed.saturating_add(move_amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupOptions<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 32 + 8)]
    pub options_state: Account<'info, OptionsState>,
    #[account(mut)] pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Process<'info> {
    #[account(mut, has_one = signer)]
    pub options_state: Account<'info, OptionsState>,
    pub signer: Signer<'info>,
    #[account(mut)] pub source_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub destination_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account] pub struct OptionsState { pub signer: Pubkey, pub allowed_program: Pubkey, pub total_processed: u64 }
#[error_code] pub enum ProcessErr { #[msg("allowed program not found")] ProgramMissing }
