// 5) market_switch
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};

declare_id!("MarketSwch111111111111111111111111111111");

#[program]
pub mod market_switch {
    use super::*;

    pub fn trade(ctx: Context<Trade>, price: u64) -> Result<()> {
        let m = &mut ctx.accounts.market;
        m.rounds += 1;

        let mut program = ctx.accounts.router_default.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            m.path_a += price;
            program = ctx.remaining_accounts[0].clone();
        } else {
            m.path_b += price;
        }

        let br = MarketBridge {
            base: ctx.accounts.base_pool.to_account_info(),
            vault: ctx.accounts.settlement_vault.to_account_info(),
        };

        let step = (price / 4) + 5;
        let mut p = price;
        while p > 0 {
            let part = if p > step { step } else { p };
            let cx = br.as_cpi(program.clone());
            br.fire(cx, part + m.rounds)?;
            p -= part;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Trade<'info> {
    #[account(init, payer = admin, space = 8 + 8 + 8 + 8)]
    pub market: Account<'info, MarketState>,
    #[account(mut)] pub admin: Signer<'info>,
    /// CHECK:
    pub base_pool: AccountInfo<'info>,
    /// CHECK:
    pub settlement_vault: AccountInfo<'info>,
    /// CHECK:
    pub router_default: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MarketState { pub rounds: u64, pub path_a: u64, pub path_b: u64 }

#[derive(Clone)]
pub struct MarketBridge<'info> { pub base: AccountInfo<'info>, pub vault: AccountInfo<'info> }

impl<'info> MarketBridge<'info> {
    pub fn as_cpi(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, MarketBridge<'info>> {
        CpiContext::new(program, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![AccountMeta::new_readonly(*self.base.key, false), AccountMeta::new(*self.vault.key, false)]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![p.clone(), self.base.clone(), self.vault.clone()]
    }
    pub fn fire(&self, ctx: CpiContext<'_, '_, '_, 'info, MarketBridge<'info>>, v: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: v.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
