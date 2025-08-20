// (2) PubkeyMappedProgram: 状態に保存した Pubkey に一致する program を remaining_accounts から検索して使用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("PkMapProg2222222222222222222222222222222");

#[program]
pub mod pubkey_mapped_program {
    use super::*;
    pub fn set_route(ctx: Context<SetRoute>, target_program_key: Pubkey) -> Result<()> {
        let route_config = &mut ctx.accounts.route_config;
        route_config.admin = ctx.accounts.admin.key();
        route_config.program_key = target_program_key;
        Ok(())
    }

    pub fn hop(ctx: Context<Hop>, move_amount: u64) -> Result<()> {
        let wanted_key = ctx.accounts.route_config.program_key;
        let mut selected_program: Option<AccountInfo> = None;

        for account_info_item in ctx.remaining_accounts.iter() {
            if account_info_item.key() == wanted_key {
                selected_program = Some(account_info_item.clone());
                break;
            }
        }
        let selected_program = selected_program.ok_or(RouteErr::ProgramNotFound)?;

        token::transfer(
            CpiContext::new(
                selected_program,
                Transfer {
                    from: ctx.accounts.from_account.to_account_info(),
                    to: ctx.accounts.to_account.to_account_info(),
                    authority: ctx.accounts.admin.to_account_info(),
                },
            ),
            move_amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetRoute<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32)]
    pub route_config: Account<'info, RouteConfig>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Hop<'info> {
    pub route_config: Account<'info, RouteConfig>,
    pub admin: Signer<'info>,
    #[account(mut)] pub from_account: Account<'info, TokenAccount>,
    #[account(mut)] pub to_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account] pub struct RouteConfig { pub admin: Pubkey, pub program_key: Pubkey }
#[error_code] pub enum RouteErr { #[msg("program not found in remaining accounts")] ProgramNotFound }
