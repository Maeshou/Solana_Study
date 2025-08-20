use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("ProgAndAi1111111111111111111111111111111");

#[program]
pub mod demo_program_and_accountinfo_mix {
    use super::*;
    pub fn mix(ctx: Context<MixCall>, val: u64) -> Result<()> {
        // Program<'info, SomeProgram> は持っているが、実行に使うのは AccountInfo 側
        let mut dynamic = ctx.accounts.fallback_prog_ai.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            dynamic = ctx.remaining_accounts[0].clone();      // ← 差し替え
        }

        let p = Pair { x: ctx.accounts.x.clone(), y: ctx.accounts.y.clone() };
        let cx = CpiContext::new(dynamic.clone(), p.clone());

        let ix = Instruction {
            program_id: *cx.program.key,                     // ← ここが可変
            accounts: p.metas(),
            data: (val ^ 0x55).to_le_bytes().to_vec(),
        };
        invoke(&ix, &p.infos(&cx.program))?;
        Ok(())
    }
}

#[account]
pub struct Dummy { pub n: u64 }

#[derive(Accounts)]
pub struct MixCall<'info> {
    /// CHECK:
    pub x: AccountInfo<'info>,
    /// CHECK:
    pub y: AccountInfo<'info>,
    // 形式上は何かの Program を要求しているが、この値は使わない（ミスパターン）
    pub declared_prog: Program<'info, System>,
    /// CHECK:
    pub fallback_prog_ai: AccountInfo<'info>,
}

#[derive(Clone)]
pub struct Pair<'info> { pub x: AccountInfo<'info>, pub y: AccountInfo<'info> }
impl<'info> Pair<'info> {
    fn metas(&self) -> Vec<AccountMeta> {
        vec![AccountMeta::new(*self.x.key, false), AccountMeta::new_readonly(*self.y.key, false)]
    }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.x.clone(), self.y.clone()] }
}
