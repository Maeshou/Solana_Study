// 8) drip_scheduler: 指定回数だけ間引いて送る（every_k）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("DripSched1111111111111111111111111111111");

#[program]
pub mod drip_scheduler {
    use super::*;
    pub fn drip(ctx: Context<Drip>, total: u64, every_k: u64) -> Result<()> {
        let st = &mut ctx.accounts.plan;
        st.executions += 1;

        let mut program = ctx.accounts.schedule_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            st.flow_a += total;
            program = ctx.remaining_accounts[0].clone();
        } else {
            st.flow_b += total;
        }

        let mut sent = 0u64;
        let mut tick = 1u64;
        let br = DripBridge { from: ctx.accounts.from_buf.to_account_info(), to: ctx.accounts.to_buf.to_account_info() };
        while sent < total {
            if (tick % every_k) > 0 {
                let size = if total - sent > every_k { every_k } else { total - sent };
                let cx = br.as_cpi(program.clone());
                br.drop(cx, size + st.executions)?;
                sent += size;
            }
            tick += 1;
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Drip<'info> {
    #[account(init, payer = owner, space = 8 + 8 + 8 + 8)]
    pub plan: Account<'info, DripPlan>,
    #[account(mut)] pub owner: Signer<'info>,
    /// CHECK:
    pub from_buf: AccountInfo<'info>,
    /// CHECK:
    pub to_buf: AccountInfo<'info>,
    /// CHECK:
    pub schedule_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct DripPlan { pub executions: u64, pub flow_a: u64, pub flow_b: u64 }
#[derive(Clone)] pub struct DripBridge<'info> { pub from: AccountInfo<'info>, pub to: AccountInfo<'info> }
impl<'info> DripBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, DripBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.from.key, false), AccountMeta::new(*self.to.key, false)] }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.from.clone(), self.to.clone()] }
    pub fn drop(&self, ctx: CpiContext<'_, '_, '_, 'info, DripBridge<'info>>, v: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: v.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
