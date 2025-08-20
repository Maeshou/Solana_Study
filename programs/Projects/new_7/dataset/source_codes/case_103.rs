// 4) mosaic_spreader: メタ順を回転（else無し）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};
declare_id!("MosaicSprd111111111111111111111111111111");

#[program]
pub mod mosaic_spreader {
    use super::*;
    pub fn spread(ctx: Context<Spread>, n: u64) -> Result<()> {
        let st=&mut ctx.accounts.ms;
        st.round += 1;

        let mut program=ctx.accounts.router_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            program = ctx.remaining_accounts[0].clone();
            st.alt += 1;
        }

        let br = MsBridge{ x:ctx.accounts.xbuf.to_account_info(), y:ctx.accounts.ybuf.to_account_info(), z:ctx.accounts.zbuf.to_account_info() };
        let mut i=0u64;
        while i<n {
            let cx=br.as_cpi(program.clone());
            if (i & 1) > 0 { br.cast_yz(cx, (i+st.round) as u64)?; }
            if (i & 1) == 0 { br.cast_zx(cx, (i+st.round) as u64)?; }
            i += 1;
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Spread<'info>{
    #[account(init,payer=owner,space=8+8+8)]
    pub ms: Account<'info, MsState>,
    #[account(mut)] pub owner: Signer<'info>,
    /// CHECK:
    pub xbuf: AccountInfo<'info>,
    /// CHECK:
    pub ybuf: AccountInfo<'info>,
    /// CHECK:
    pub zbuf: AccountInfo<'info>,
    /// CHECK:
    pub router_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct MsState{ pub round:u64, pub alt:u64 }
#[derive(Clone)] pub struct MsBridge<'info>{ pub x:AccountInfo<'info>, pub y:AccountInfo<'info>, pub z:AccountInfo<'info> }
impl<'info> MsBridge<'info>{
    pub fn as_cpi(&self,p:AccountInfo<'info>)->CpiContext<'_, '_, '_, 'info, MsBridge<'info>>{ CpiContext::new(p, self.clone()) }
    fn metas_yz(&self)->Vec<AccountMeta>{ vec![AccountMeta::new_readonly(*self.y.key,false), AccountMeta::new(*self.z.key,false)] }
    fn metas_zx(&self)->Vec<AccountMeta>{ vec![AccountMeta::new_readonly(*self.z.key,false), AccountMeta::new(*self.x.key,false)] }
    fn infos(&self,p:&AccountInfo<'info>)->Vec<AccountInfo<'info>>{ vec![p.clone(), self.x.clone(), self.y.clone(), self.z.clone()] }
    pub fn cast_yz(&self, cx:CpiContext<'_, '_, '_, 'info, MsBridge<'info>>, v:u64)->Result<()>{
        let ix=Instruction{ program_id:*cx.program.key, accounts:self.metas_yz(), data:v.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&cx.program))?; Ok(())
    }
    pub fn cast_zx(&self, cx:CpiContext<'_, '_, '_, 'info, MsBridge<'info>>, v:u64)->Result<()>{
        let ix=Instruction{ program_id:*cx.program.key, accounts:self.metas_zx(), data:v.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&cx.program))?; Ok(())
    }
}
