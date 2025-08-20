// 9) shard_funnel: シャードIDごとにバケットへ集約してから1回送信（集計→一括CPI）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("ShardFunnel11111111111111111111111111111");

#[program]
pub mod shard_funnel {
    use super::*;
    pub fn funnel(ctx: Context<Funnel>, base: u64, shards: u64) -> Result<()> {
        let st = &mut ctx.accounts.funnel_state;
        st.round += 1;

        let mut program = ctx.accounts.pipe_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            st.path_a += base;
            program = ctx.remaining_accounts[0].clone();
        } else {
            st.path_b += base;
        }

        let mut acc = 0u64;
        let mut i = 0u64;
        while i < shards {
            let bucket = base + (i * st.round);
            acc += bucket;
            i += 1;
        }
        let br = FunnelBridge { inq: ctx.accounts.in_q.to_account_info(), outq: ctx.accounts.out_q.to_account_info() };
        let cx = br.as_cpi(program.clone());
        br.send(cx, acc)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Funnel<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 8 + 8)]
    pub funnel_state: Account<'info, FunnelState>,
    #[account(mut)] pub payer: Signer<'info>,
    /// CHECK:
    pub in_q: AccountInfo<'info>,
    /// CHECK:
    pub out_q: AccountInfo<'info>,
    /// CHECK:
    pub pipe_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct FunnelState { pub round: u64, pub path_a: u64, pub path_b: u64 }
#[derive(Clone)] pub struct FunnelBridge<'info> { pub inq: AccountInfo<'info>, pub outq: AccountInfo<'info> }
impl<'info> FunnelBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, FunnelBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new_readonly(*self.inq.key, false), AccountMeta::new(*self.outq.key, false)] }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.inq.clone(), self.outq.clone()] }
    pub fn send(&self, ctx: CpiContext<'_, '_, '_, 'info, FunnelBridge<'info>>, n: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: n.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
