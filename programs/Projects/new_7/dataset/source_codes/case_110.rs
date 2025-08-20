// 7) ladder_upgrade: 段階的に倍率を上げながら複数回送る（倍率は状態とseedで決定）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("LadderUp11111111111111111111111111111111");

#[program]
pub mod ladder_upgrade {
    use super::*;
    pub fn apply(ctx: Context<Apply>, seed: u64) -> Result<()> {
        let st = &mut ctx.accounts.cfg;
        st.levels += 1;

        let mut program = ctx.accounts.pipe_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            st.flow_a += seed;
            program = ctx.remaining_accounts[0].clone();
        } else {
            st.flow_b += seed;
        }

        let base = (seed & 7) + 2;
        let br = LadderBridge { src: ctx.accounts.src_q.to_account_info(), dst: ctx.accounts.dst_q.to_account_info() };
        let mut i = 0u64;
        while i < base {
            let amt = (base + i) * (st.levels as u64);
            let cx = br.as_cpi(program.clone());
            br.push(cx, amt)?;
            i += 1;
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Apply<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 8 + 8)]
    pub cfg: Account<'info, LadderCfg>,
    #[account(mut)] pub payer: Signer<'info>,
    /// CHECK:
    pub src_q: AccountInfo<'info>,
    /// CHECK:
    pub dst_q: AccountInfo<'info>,
    /// CHECK:
    pub pipe_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct LadderCfg { pub levels: u64, pub flow_a: u64, pub flow_b: u64 }
#[derive(Clone)] pub struct LadderBridge<'info> { pub src: AccountInfo<'info>, pub dst: AccountInfo<'info> }
impl<'info> LadderBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, LadderBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new_readonly(*self.src.key, false), AccountMeta::new(*self.dst.key, false)] }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.src.clone(), self.dst.clone()] }
    pub fn push(&self, ctx: CpiContext<'_, '_, '_, 'info, LadderBridge<'info>>, v: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: v.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
