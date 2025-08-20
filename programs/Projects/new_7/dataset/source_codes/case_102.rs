// 5) watermark_queue: 可変プレフィックス＋長さフィールド追加で送信（else無し）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
declare_id!("WatermarkQ111111111111111111111111111111");

#[program]
pub mod watermark_queue {
    use super::*;
    pub fn mark(ctx: Context<Mark>, v: u64) -> Result<()> {
        let st=&mut ctx.accounts.meta;
        st.epoch += 1;

        let mut program=ctx.accounts.pipe_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            program = ctx.remaining_accounts[0].clone();
            st.alt += v;
        }

        let mut data = Vec::with_capacity(24);
        data.extend_from_slice(&st.epoch.to_le_bytes());
        data.extend_from_slice(&(8u64).to_le_bytes());
        data.extend_from_slice(&v.to_le_bytes());

        let br = MarkBridge{ a:ctx.accounts.src.to_account_info(), b:ctx.accounts.dst.to_account_info() };
        let cx = br.as_cpi(program.clone());
        br.send(cx, data)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Mark<'info>{
    #[account(init,payer=owner,space=8+8+8)]
    pub meta: Account<'info, WmMeta>,
    #[account(mut)] pub owner: Signer<'info>,
    /// CHECK:
    pub src: AccountInfo<'info>,
    /// CHECK:
    pub dst: AccountInfo<'info>,
    /// CHECK:
    pub pipe_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct WmMeta{ pub epoch:u64, pub alt:u64 }
#[derive(Clone)] pub struct MarkBridge<'info>{ pub a:AccountInfo<'info>, pub b:AccountInfo<'info> }
impl<'info> MarkBridge<'info>{
    pub fn as_cpi(&self,p:AccountInfo<'info>)->CpiContext<'_, '_, '_, 'info, MarkBridge<'info>>{ CpiContext::new(p, self.clone()) }
    fn metas(&self)->Vec<AccountMeta>{ vec![AccountMeta::new(*self.a.key,false), AccountMeta::new_readonly(*self.b.key,false)] }
    fn infos(&self,p:&AccountInfo<'info>)->Vec<AccountInfo<'info>>{ vec![p.clone(), self.a.clone(), self.b.clone()] }
    pub fn send(&self, cx:CpiContext<'_, '_, '_, 'info, MarkBridge<'info>>, bytes:Vec<u8>)->Result<()>{
        let ix=Instruction{ program_id:*cx.program.key, accounts:self.metas(), data:bytes };
        invoke(&ix, &self.infos(&cx.program))?; Ok(())
    }
}
