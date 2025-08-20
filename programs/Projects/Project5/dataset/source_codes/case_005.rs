// ============================================================================
// 1) Loyalty Ledger（ポイント台帳） — constraint + require_keys_neq!
// ============================================================================
declare_id!("LL11111111111111111111111111111111");
use anchor_lang::prelude::*;

#[program]
pub mod loyalty_ledger {
    use super::*;

    pub fn init_ledger(ctx: Context<InitLedger>, cap: u64, tier: u8) -> Result<()> {
        let l = &mut ctx.accounts.ledger;
        l.owner = ctx.accounts.member.key();
        l.tier = tier;
        l.cap = cap;

        let c = &mut ctx.accounts.counters;
        c.accum = 0;
        c.ops = 0;
        c.hot = true;

        let s = &mut ctx.accounts.settings;
        s.fee = 3;
        s.open = true;
        s.bump = *ctx.bumps.get("settings").unwrap();
        Ok(())
    }

    pub fn credit_points(ctx: Context<CreditPoints>, add: u32) -> Result<()> {
        // ここで二重可変アカウントを排除
        require_keys_neq!(ctx.accounts.ledger.key(), ctx.accounts.counters.key(), LLError::SameAccount);
        require_keys_neq!(ctx.accounts.ledger.key(), ctx.accounts.settings.key(), LLError::SameAccount);

        let mut i = 0u32;
        while i < add {
            ctx.accounts.counters.accum = ctx.accounts.counters.accum.saturating_add(1);
            ctx.accounts.counters.ops = ctx.accounts.counters.ops.saturating_add(1);
            i += 1;
        }

        if (ctx.accounts.counters.accum as u64) > ctx.accounts.ledger.cap {
            ctx.accounts.settings.open = false;
            ctx.accounts.counters.hot = false;
            ctx.accounts.counters.ops = ctx.accounts.counters.ops.saturating_add(10);
            msg!("Cap exceeded: accum={}, cap={}", ctx.accounts.counters.accum, ctx.accounts.ledger.cap);
        } else {
            ctx.accounts.settings.fee = ctx.accounts.settings.fee.saturating_add(1);
            ctx.accounts.counters.hot = true;
            ctx.accounts.counters.ops = ctx.accounts.counters.ops.saturating_add(2);
            msg!("Within cap: accum={}, fee={}", ctx.accounts.counters.accum, ctx.accounts.settings.fee);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLedger<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 1 + 8)]
    pub ledger: Account<'info, Ledger>,
    #[account(init, payer = payer, space = 8 + 8 + 4 + 1)]
    pub counters: Account<'info, Counters>,
    #[account(init, seeds = [b"settings", payer.key().as_ref()], bump, payer = payer, space = 8 + 4 + 1 + 1)]
    pub settings: Account<'info, Settings>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub member: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreditPoints<'info> {
    #[account(mut, constraint = ledger.owner == member.key())]
    pub ledger: Account<'info, Ledger>,
    #[account(mut, constraint = counters.key() != settings.key(), error = LLError::SameAccount)]
    pub counters: Account<'info, Counters>,
    #[account(mut)]
    pub settings: Account<'info, Settings>,
    pub member: Signer<'info>,
}

#[account]
pub struct Ledger { pub owner: Pubkey, pub tier: u8, pub cap: u64 }
#[account]
pub struct Counters { pub accum: u64, pub ops: u32, pub hot: bool }
#[account]
pub struct Settings { pub fee: u32, pub open: bool, pub bump: u8 }

#[error_code]
pub enum LLError { #[msg("same account passed twice")] SameAccount }
