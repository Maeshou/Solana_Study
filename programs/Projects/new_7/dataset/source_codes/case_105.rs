// 2) shard_writer: シャード幅に応じて倍々書き込み（else無し）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
declare_id!("ShardWritr1111111111111111111111111111111");

#[program]
pub mod shard_writer {
    use super::*;
    pub fn write(ctx: Context<Write>, base: u64, shards: u64) -> Result<()> {
        let st=&mut ctx.accounts.state;
        st.round += 1;

        let mut program = ctx.accounts.default_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            program = ctx.remaining_accounts[0].clone();
            st.alt_hits += 1;
        }
        let br = ShardBridge{ src:ctx.accounts.src_q.to_account_info(), dst:ctx.accounts.dst_q.to_account_info() };

        let mut i=0u64;
        let mut val=base;
        while i<shards {
            let cx=br.as_cpi(program.clone());
            br.push(cx, val + st.round)?;
            val = val.saturating_mul(2);
            i += 1;
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Write<'info>{
    #[account(init,payer=payer,space=8+8+8)]
    pub state: Account<'info, ShStat>,
    #[account(mut)] pub payer: Signer<'info>,
    /// CHECK:
    pub src_q: AccountInfo<'info>,
    /// CHECK:
    pub dst_q: AccountInfo<'info>,
    /// CHECK:
    pub default_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct ShStat{ pub round:u64, pub alt_hits:u64 }
#[derive(Clone)] pub struct ShardBridge<'info>{ pub src:AccountInfo<'info>, pub dst:AccountInfo<'info> }
impl<'info> ShardBridge<'info>{
    pub fn as_cpi(&self, p:AccountInfo<'info>)->CpiContext<'_, '_, '_, 'info, ShardBridge<'info>>{ CpiContext::new(p, self.clone()) }
    fn metas(&self)->Vec<AccountMeta>{ vec![AccountMeta::new(*self.src.key,false), AccountMeta::new(*self.dst.key,false)] }
    fn infos(&self,p:&AccountInfo<'info>)->Vec<AccountInfo<'info>>{ vec![p.clone(), self.src.clone(), self.dst.clone()] }
    pub fn push(&self, cx:CpiContext<'_, '_, '_, 'info, ShardBridge<'info>>, x:u64)->Result<()>{
        let ix=Instruction{ program_id:*cx.program.key, accounts:self.metas(), data:x.to_le_bytes().to_vec() };
        invoke(&ix,&self.infos(&cx.program))?; Ok(())
    }
}
