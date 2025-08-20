use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("BrdgImpl111111111111111111111111111111111");

#[program]
pub mod demo_bridge_impl_basic {
    use super::*;
    pub fn trigger(ctx: Context<Trigger>, code: u64) -> Result<()> {
        // 呼び先プログラムを実行時に差し替え可能
        let mut target = ctx.accounts.hint_program.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            target = ctx.remaining_accounts[0].clone();
        }

        // impl で CpiContext を組み立て
        let br = NoticeLine {
            outlet: ctx.accounts.outlet.to_account_info(),
            actor:  ctx.accounts.actor.to_account_info(),
        };
        let cx = br.as_cpi(target.clone()); // ← program を外部から注入
        br.post(cx, code.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Trigger<'info> {
    /// CHECK:
    pub outlet: AccountInfo<'info>,
    /// CHECK:
    pub actor: AccountInfo<'info>,
    /// CHECK:
    pub hint_program: AccountInfo<'info>,
}

#[derive(Clone)]
pub struct NoticeLine<'info> {
    pub outlet: AccountInfo<'info>,
    pub actor: AccountInfo<'info>,
}

impl<'info> NoticeLine<'info> {
    pub fn as_cpi(
        &self,
        program: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, NoticeLine<'info>> {
        CpiContext::new(program, self.clone())
    }

    fn metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(*self.outlet.key, false),
            AccountMeta::new_readonly(*self.actor.key, false),
        ]
    }

    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![p.clone(), self.outlet.clone(), self.actor.clone()]
    }

    pub fn post(
        &self,
        cx: CpiContext<'_, '_, '_, 'info, NoticeLine<'info>>,
        data: Vec<u8>,
    ) -> Result<()> {
        let ix = Instruction {
            program_id: *cx.program.key,            // ← 差し替えIDがそのまま使用
            accounts: self.metas(),
            data,
        };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
