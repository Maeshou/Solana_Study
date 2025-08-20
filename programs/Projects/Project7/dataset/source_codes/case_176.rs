// 9) ラウンドごとに even_round フラグを反転し、true のとき AccountInfo 経路を使用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("EvenRoundFlip9999999999999999999999999999");

#[program]
pub mod even_round_flip_selector {
    use super::*;
    pub fn create(ctx: Context<CreateFlip>, unit: u64, roof: u64) -> Result<()> {
        let f = &mut ctx.accounts.flow;
        f.operator = ctx.accounts.operator.key();
        f.unit = unit;
        if f.unit < 1 { f.unit = 1; }
        f.roof = roof;
        if f.roof < f.unit { f.roof = f.unit; }
        f.total = 0;
        f.even_round = true;
        Ok(())
    }

    pub fn drive(ctx: Context<DriveFlip>, reps: u8) -> Result<()> {
        let f = &mut ctx.accounts.flow;
        let mut r: u8 = 0;
        while r < reps {
            let mut amount = f.unit;
            if amount < 1 { amount = 1; }
            let plan = f.total.saturating_add(amount);
            if plan > f.roof { return Err(FlipErr::Roof.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if f.even_round { program_ai = ctx.accounts.alt_path.clone(); }

            token::approve(ctx.accounts.a(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.t(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.r(program_ai))?;

            f.total = plan;
            f.even_round = !f.even_round;
            r = r.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateFlip<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub flow: Account<'info, FlipState>,
    #[account(mut)] pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DriveFlip<'info> {
    #[account(mut, has_one = operator)]
    pub flow: Account<'info, FlipState>,
    pub operator: Signer<'info>,
    #[account(mut)] pub stage_in: Account<'info, TokenAccount>,
    #[account(mut)] pub stage_out: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub alt_path: AccountInfo<'info>,
}
impl<'info> DriveFlip<'info> {
    fn a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.stage_in.to_account_info(), delegate: self.stage_out.to_account_info(), authority: self.operator.to_account_info() };
        CpiContext::new(p, a)
    }
    fn t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.stage_in.to_account_info(), to: self.stage_out.to_account_info(), authority: self.operator.to_account_info() };
        CpiContext::new(p, t)
    }
    fn r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.stage_in.to_account_info(), authority: self.operator.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct FlipState { pub operator: Pubkey, pub unit: u64, pub roof: u64, pub total: u64, pub even_round: bool }
#[error_code] pub enum FlipErr { #[msg("roof reached")] Roof }
