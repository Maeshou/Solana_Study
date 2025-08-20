// 10) BalanceWeightedRouter: 送信先の流動性がしきい値未満なら外部プログラムを優先
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("BalanceWeightedAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod balance_weighted_router {
    use super::*;
    pub fn configure(ctx: Context<ConfigureWeighted>, unit_value: u64, cap_value: u64, min_liquidity: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.admin = ctx.accounts.admin.key();
        pool.unit_value = unit_value.max(1);
        pool.cap_value = cap_value.max(pool.unit_value);
        pool.min_liquidity = min_liquidity;
        pool.total_value = 0;
        Ok(())
    }
    pub fn execute(ctx: Context<ExecuteWeighted>, repeats: u8) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let mut loop_cursor: u8 = 0;

        while loop_cursor < repeats {
            let next = pool.total_value.saturating_add(pool.unit_value);
            if next > pool.cap_value { return Err(BalErr::Cap.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            if ctx.accounts.destination.amount < pool.min_liquidity { program_account_info = ctx.accounts.external_program.clone(); }

            token::approve(ctx.accounts.ap(program_account_info.clone()), pool.unit_value)?;
            token::transfer(ctx.accounts.tr(program_account_info.clone()), pool.unit_value)?;
            token::revoke(ctx.accounts.rv(program_account_info))?;

            pool.total_value = next;
            loop_cursor = loop_cursor.saturating_add(1);
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct ConfigureWeighted<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub pool: Account<'info, WeightedState>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ExecuteWeighted<'info> {
    #[account(mut, has_one = admin)]
    pub pool: Account<'info, WeightedState>,
    pub admin: Signer<'info>,
    #[account(mut)] pub source: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}
impl<'info> ExecuteWeighted<'info> {
    fn ap(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> { 
        CpiContext::new(p, Approve { to: self.source.to_account_info(), delegate: self.destination.to_account_info(), authority: self.admin.to_account_info() })
    }
    fn tr(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> { 
        CpiContext::new(p, Transfer { from: self.source.to_account_info(), to: self.destination.to_account_info(), authority: self.admin.to_account_info() })
    }
    fn rv(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> { 
        CpiContext::new(p, Revoke { source: self.source.to_account_info(), authority: self.admin.to_account_info() })
    }
}
#[account] pub struct WeightedState { pub admin: Pubkey, pub unit_value: u64, pub cap_value: u64, pub min_liquidity: u64, pub total_value: u64 }
#[error_code] pub enum BalErr { #[msg("cap reached")] Cap }
