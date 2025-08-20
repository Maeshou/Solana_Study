// 6) scoreboard_router
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};

declare_id!("ScoreBoardR111111111111111111111111111111");

#[program]
pub mod scoreboard_router {
    use super::*;

    pub fn submit(ctx: Context<Submit>, pts: u64) -> Result<()> {
        let s = &mut ctx.accounts.board;
        s.submits += 1;

        let mut program = ctx.accounts.default_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            s.path_a += pts;
            program = ctx.remaining_accounts[0].clone();
        } else {
            s.path_b += pts;
        }

        let br = ScoreBridge {
            player: ctx.accounts.player_acct.to_account_info(),
            table: ctx.accounts.table_acct.to_account_info(),
        };

        let block = (pts / 2) + 3;
        let mut left = pts;
        while left > 0 {
            let take = if left > block { block } else { left };
            let cx = br.as_cpi(program.clone());
            br.push(cx, take + s.submits)?;
            left -= take;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Submit<'info> {
    #[account(init, payer = owner, space = 8 + 8 + 8 + 8)]
    pub board: Account<'info, BoardState>,
    #[account(mut)] pub owner: Signer<'info>,
    /// CHECK:
    pub player_acct: AccountInfo<'info>,
    /// CHECK:
    pub table_acct: AccountInfo<'info>,
    /// CHECK:
    pub default_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BoardState { pub submits: u64, pub path_a: u64, pub path_b: u64 }

#[derive(Clone)]
pub struct ScoreBridge<'info> { pub player: AccountInfo<'info>, pub table: AccountInfo<'info> }

impl<'info> ScoreBridge<'info> {
    pub fn as_cpi(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, ScoreBridge<'info>> {
        CpiContext::new(program, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![AccountMeta::new_readonly(*self.player.key, false), AccountMeta::new(*self.table.key, false)]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![p.clone(), self.player.clone(), self.table.clone()]
    }
    pub fn push(&self, ctx: CpiContext<'_, '_, '_, 'info, ScoreBridge<'info>>, n: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: n.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
