use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("CtxOnAcc11111111111111111111111111111111");

#[program]
pub mod demo_accounts_impl_ctx {
    use super::*;
    pub fn run(ctx: Context<Runner>, payload: u64) -> Result<()> {
        let (cx, pack) = ctx.accounts.build_ctx(&ctx.remaining_accounts)?;
        let ix = Instruction {
            program_id: *cx.program.key,          // ← 動的に注入された program
            accounts: pack.metas(),
            data: payload.to_le_bytes().to_vec(),
        };
        invoke(&ix, &pack.infos(&cx.program))?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Runner<'info> {
    /// CHECK:
    pub board: AccountInfo<'info>,
    /// CHECK:
    pub who: AccountInfo<'info>,
    /// CHECK:
    pub default_program: AccountInfo<'info>,
}

#[derive(Clone)]
pub struct Pack<'info> {
    pub board: AccountInfo<'info>,
    pub who: AccountInfo<'info>,
}

impl<'info> Pack<'info> {
    fn metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(*self.board.key, false),
            AccountMeta::new_readonly(*self.who.key, false),
        ]
    }
    fn infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![program.clone(), self.board.clone(), self.who.clone()]
    }
}

impl<'info> Runner<'info> {
    pub fn build_ctx(
        &self,
        rem: &[AccountInfo<'info>],
    ) -> Result<(CpiContext<'_, '_, '_, 'info, Pack<'info>>, Pack<'info>)> {
        let mut program = self.default_program.to_account_info();
        if !rem.is_empty() {
            program = rem[0].clone();              // ← 差し替え可能
        }
        let pack = Pack { board: self.board.clone(), who: self.who.clone() };
        let cx = CpiContext::new(program, pack.clone());
        Ok((cx, pack))
    }
}
