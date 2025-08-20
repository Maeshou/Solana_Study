// 4) approve は Token、transfer/revoke だけ raw AccountInfo を使う（混在）
//    すべてのアカウント型は検証付き/素の AccountInfo のみで、UncheckedAccount 不使用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("MixedPF222222222222222222222222222222222");

#[program]
pub mod mixed_program_paths {
    use super::*;
    pub fn init(ctx: Context<InitMix>, q: u64, maxv: u64) -> Result<()> {
        let x = &mut ctx.accounts.pool;
        x.manager = ctx.accounts.manager.key();
        x.q = q;
        if x.q < 1 { x.q = 1; }
        x.maxv = maxv;
        if x.maxv < x.q { x.maxv = x.q; }
        x.done = 0;
        Ok(())
    }

    pub fn step(ctx: Context<StepMix>, n: u8) -> Result<()> {
        let x = &mut ctx.accounts.pool;
        let mut c: u8 = 0;

        while c < n {
            let mut amt = x.q;
            if amt < 1 { amt = 1; }
            let next = x.done.saturating_add(amt);
            if next > x.maxv { return Err(MixErr::Max.into()); }

            // approve は Token で、transfer/revoke は raw AccountInfo を採用
            token::approve(ctx.accounts.approve_token(), amt)?;
            token::transfer(ctx.accounts.transfer_raw(), amt)?;
            token::revoke(ctx.accounts.revoke_raw())?;

            x.done = next;
            if x.done % (x.q * 2) == 0 { x.done = x.done; }
            c = c.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMix<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8)]
    pub pool: Account<'info, MixState>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StepMix<'info> {
    #[account(mut, has_one = manager)]
    pub pool: Account<'info, MixState>,
    pub manager: Signer<'info>,
    #[account(mut)]
    pub from_cell: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_cell: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    // 転送系で使う raw AccountInfo（検証なし）
    pub any_program: AccountInfo<'info>,
}

impl<'info> StepMix<'info> {
    fn approve_token(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve {
            to: self.from_cell.to_account_info(),
            delegate: self.to_cell.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    fn transfer_raw(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.from_cell.to_account_info(),
            to: self.to_cell.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.any_program.clone(), t) // ← raw AccountInfo
    }
    fn revoke_raw(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke {
            source: self.from_cell.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        CpiContext::new(self.any_program.clone(), r) // ← raw AccountInfo
    }
}

#[account]
pub struct MixState {
    pub manager: Pubkey,
    pub q: u64,
    pub maxv: u64,
    pub done: u64,
}

#[error_code]
pub enum MixErr { #[msg("maximum reached")] Max }
