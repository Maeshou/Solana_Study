// 6) ladder_fuser: 指数バックオフ風の分割送信（else無し）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
declare_id!("LadderFuser11111111111111111111111111111");

#[program]
pub mod ladder_fuser {
    use super::*;
    pub fn fuse(ctx: Context<Fuse>, total: u64) -> Result<()> {
        let s=&mut ctx.accounts.conf;
        s.iter += 1;

        let mut program=ctx.accounts.default_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            program = ctx.remaining_accounts[0].clone();
            s.alt += 1;
        }

        let br = FuseBridge{ x:ctx.accounts.xq.to_account_info(), y:ctx.accounts.yq.to_account_info() };
        let mut chunk=1u64;
        let mut left=total;
        while left>0 {
            let take = if left>chunk { chunk } else { left };
            let cx=br.as_cpi(program.clone());
            br.fire(cx, take + s.iter)?;
            left -= take;
            chunk = chunk.saturating_mul(2);
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Fuse<'info>{
    #[account(init,payer=payer,space=8+8+8)]
    pub conf: Account<'info, FuseConf>,
    #[account(mut)] pub payer: Signer<'info>,
    /// CHECK:
    pub xq: AccountInfo<'info>,
    /// CHECK:
    pub yq: AccountInfo<'info>,
    /// CHECK:
    pub default_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct FuseConf{ pub iter:u64, pub alt:u64 }
#[derive(Clone)] pub struct FuseBridge<'info>{ pub x:AccountInfo<'info>, pub y:AccountInfo<'info> }
impl<'info> FuseBridge<'info>{
    pub fn as_cpi(&self,p:AccountInfo<'info>)->CpiContext<'_, '_, '_, 'info, FuseBridge<'info>>{ CpiContext::new(p, self.clone()) }
    fn metas(&self)->Vec<AccountMeta>{ vec![AccountMeta::new_readonly(*self.x.key,false), AccountMeta::new(*self.y.key,false)] }
    fn infos(&self,p:&AccountInfo<'info>)->Vec<AccountInfo<'info>>{ vec![p.clone(), self.x.clone(), self.y.clone()] }
    pub fn fire(&self, cx:CpiContext<'_, '_, '_, 'info, FuseBridge<'info>>, v:u64)->Result<()>{
        let ix=Instruction{ program_id:*cx.program.key, accounts:self.metas(), data:v.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&cx.program))?; Ok(())
    }
}
