// 4) pet_training_camp: しつけログと外部評価呼び出し（ループ→分岐→分岐）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{AccountMeta, Instruction}};
declare_id!("PetTrainp1111111111111111111111111111111");

#[program]
pub mod pet_training_camp {
    use super::*;
    pub fn train(ctx: Context<Train>, steps: u64) -> Result<()> {
        let p = &mut ctx.accounts.plan;
        let mut callp = ctx.accounts.judge_prog.to_account_info();

        for _ in 0..(steps % 5 + 1) { p.stamps += 1; }
        if p.stamps > 6 { p.flags ^= steps; }
        if ctx.remaining_accounts.len() > 0 { callp = ctx.remaining_accounts[0].clone(); }

        let br = JudgeBridge { kennel: ctx.accounts.kennel.to_account_info(), owner: ctx.accounts.owner.to_account_info() };
        let cx = br.as_cpi(callp.clone());
        br.score(cx, steps.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Train<'info> {
    #[account(mut)]
    pub plan: Account<'info, TrainPlan>,
    /// CHECK:
    pub kennel: AccountInfo<'info>,
    /// CHECK:
    pub owner: AccountInfo<'info>,
    /// CHECK:
    pub judge_prog: AccountInfo<'info>,
}
#[account] pub struct TrainPlan { pub stamps: u64, pub flags: u64 }

#[derive(Clone)]
pub struct JudgeBridge<'info> { pub kennel: AccountInfo<'info>, pub owner: AccountInfo<'info> }
impl<'info> JudgeBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, JudgeBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.kennel.key, false), AccountMeta::new_readonly(*self.owner.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.kennel.clone(), self.owner.clone()] }
    pub fn score(&self, cx: CpiContext<'_, '_, '_, 'info, JudgeBridge<'info>>, bytes: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: bytes };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
