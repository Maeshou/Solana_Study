use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke_signed};

declare_id!("WithSigner1111111111111111111111111111111");

#[program]
pub mod demo_with_signer_dynamic_program {
    use super::*;
    pub fn dispatch(ctx: Context<Dispatch>, n: u64) -> Result<()> {
        let mut program = ctx.accounts.route_default.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            program = ctx.remaining_accounts[0].clone();     // ← 任意IDに置換
        }

        let pack = Duo { a: ctx.accounts.a.clone(), b: ctx.accounts.b.clone() };
        let cx = CpiContext::new_with_signer(program.clone(), pack.clone(), &[b"seed-a", b"seed-b"]);

        let ix = Instruction {
            program_id: *cx.program.key,                   // ← 置換先が実行対象
            accounts: pack.metas(),
            data: n.to_le_bytes().to_vec(),
        };
        invoke_signed(&ix, &pack.infos(&cx.program), &[b"seed-a", b"seed-b"])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Dispatch<'info> {
    /// CHECK:
    pub a: AccountInfo<'info>,
    /// CHECK:
    pub b: AccountInfo<'info>,
    /// CHECK:
    pub route_default: AccountInfo<'info>,
}

#[derive(Clone)]
pub struct Duo<'info> { pub a: AccountInfo<'info>, pub b: AccountInfo<'info> }
impl<'info> Duo<'info> {
    fn metas(&self) -> Vec<AccountMeta> {
        vec![AccountMeta::new(*self.a.key, false), AccountMeta::new_readonly(*self.b.key, false)]
    }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.a.clone(), self.b.clone()] }
}
