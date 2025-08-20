// 2) OperatorHoldRouter: 状態フラグ hold_external が true の間は常に外部プログラム
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("OperatorHold22222222222222222222222222222");

#[program]
pub mod operator_hold_router {
    use super::*;
    pub fn setup(ctx: Context<SetupHold>, step_units: u64, max_volume: u64, hold_external: bool) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        settings.owner = ctx.accounts.owner.key();
        settings.step_units = step_units.max(1);
        settings.max_volume = max_volume.max(settings.step_units);
        settings.total_volume = 0;
        settings.hold_external = hold_external;
        Ok(())
    }
    pub fn run(ctx: Context<RunHold>, steps: u8) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        let mut ticks: u8 = 0;

        while ticks < steps {
            let next_volume = settings.total_volume.saturating_add(settings.step_units);
            if next_volume > settings.max_volume { return Err(HoldErr::Cap.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            if settings.hold_external { program_account_info = ctx.accounts.external_path.clone(); }

            token::approve(ctx.accounts.ctx_a(program_account_info.clone()), settings.step_units)?;
            token::transfer(ctx.accounts.ctx_t(program_account_info.clone()), settings.step_units)?;
            token::revoke(ctx.accounts.ctx_r(program_account_info))?;

            settings.total_volume = next_volume;
            ticks = ticks.saturating_add(1);
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct SetupHold<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub settings: Account<'info, HoldState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RunHold<'info> {
    #[account(mut, has_one = owner)]
    pub settings: Account<'info, HoldState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub hot_wallet: Account<'info, TokenAccount>,
    #[account(mut)] pub cold_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_path: AccountInfo<'info>,
}
impl<'info> RunHold<'info> {
    fn ctx_a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(p, Approve { to: self.hot_wallet.to_account_info(), delegate: self.cold_wallet.to_account_info(), authority: self.owner.to_account_info() })
    }
    fn ctx_t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(p, Transfer { from: self.hot_wallet.to_account_info(), to: self.cold_wallet.to_account_info(), authority: self.owner.to_account_info() })
    }
    fn ctx_r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(p, Revoke { source: self.hot_wallet.to_account_info(), authority: self.owner.to_account_info() })
    }
}
#[account] pub struct HoldState { pub owner: Pubkey, pub step_units: u64, pub max_volume: u64, pub total_volume: u64, pub hold_external: bool }
#[error_code] pub enum HoldErr { #[msg("cap reached")] Cap }
