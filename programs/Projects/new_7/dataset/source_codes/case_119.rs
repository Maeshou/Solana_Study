// 4) quest_progressor
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};

declare_id!("QuestProgr1111111111111111111111111111111");

#[program]
pub mod quest_progressor {
    use super::*;

    pub fn step(ctx: Context<Step>, value: u64) -> Result<()> {
        let q = &mut ctx.accounts.progress;
        q.counter += 1;

        let mut program = ctx.accounts.alt_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            q.path_a += value;
            program = ctx.remaining_accounts[0].clone();
        } else {
            q.path_b += value;
        }

        let br = QuestBridge {
            src: ctx.accounts.progress_src.to_account_info(),
            dst: ctx.accounts.progress_dst.to_account_info(),
        };
        let cx = br.as_cpi(program.clone());
        br.apply(cx, value + q.counter)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Step<'info> {
    #[account(init, payer = owner, space = 8 + 8 + 8 + 8)]
    pub progress: Account<'info, ProgressState>,
    #[account(mut)] pub owner: Signer<'info>,
    /// CHECK:
    pub progress_src: AccountInfo<'info>,
    /// CHECK:
    pub progress_dst: AccountInfo<'info>,
    /// CHECK:
    pub alt_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProgressState { pub counter: u64, pub path_a: u64, pub path_b: u64 }

#[derive(Clone)]
pub struct QuestBridge<'info> { pub src: AccountInfo<'info>, pub dst: AccountInfo<'info> }

impl<'info> QuestBridge<'info> {
    pub fn as_cpi(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, QuestBridge<'info>> {
        CpiContext::new(program, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![AccountMeta::new_readonly(*self.src.key, false), AccountMeta::new(*self.dst.key, false)]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![p.clone(), self.src.clone(), self.dst.clone()]
    }
    pub fn apply(&self, ctx: CpiContext<'_, '_, '_, 'info, QuestBridge<'info>>, n: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: n.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
