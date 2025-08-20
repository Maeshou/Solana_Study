// 2) arena_energy_refill: エネルギー補充を外部へ転送（CpiContext）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};

declare_id!("ArenaEnerRef11111111111111111111111111111");

#[program]
pub mod arena_energy_refill {
    use super::*;
    pub fn refill(ctx: Context<Refill>, amount: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.energy += amount;

        let mut prog = ctx.accounts.refill_program.to_account_info();
        if s.energy % 2 == 0 {
            s.tick += 1;
        }
        if ctx.remaining_accounts.len() > 1 {
            prog = ctx.remaining_accounts[1].clone();
            s.switch_count += 1;
        }

        let br = RefillBridge {
            pool: ctx.accounts.energy_pool.to_account_info(),
            user: ctx.accounts.user_wallet.to_account_info(),
        };
        let cx = br.as_cpi(prog.clone());
        br.forward(cx, amount.to_le_bytes().to_vec())?;

        if s.energy > s.cap {
            s.energy = s.cap;
            let cx2 = br.as_cpi(prog.clone());
            br.forward(cx2, (1u64).to_le_bytes().to_vec())?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Refill<'info> {
    #[account(mut)]
    pub station: Account<'info, Station>,
    /// CHECK:
    pub energy_pool: AccountInfo<'info>,
    /// CHECK:
    pub user_wallet: AccountInfo<'info>,
    /// CHECK:
    pub refill_program: AccountInfo<'info>,
}

#[account]
pub struct Station {
    pub energy: u64,
    pub cap: u64,
    pub tick: u64,
    pub switch_count: u64,
}

#[derive(Clone)]
pub struct RefillBridge<'info> {
    pub pool: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
}

impl<'info> RefillBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, RefillBridge<'info>> {
        CpiContext::new(p, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.pool.key, false), AccountMeta::new(*self.user.key, false)] }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.pool.clone(), self.user.clone()] }
    pub fn forward(&self, cx: CpiContext<'_, '_, '_, 'info, RefillBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
