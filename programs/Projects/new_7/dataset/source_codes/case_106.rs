// 10) tally_spreader: 合計を移動平均で均しつつ連続送信、メタにreadonly×2を混在
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("TallySprd1111111111111111111111111111111");

#[program]
pub mod tally_spreader {
    use super::*;
    pub fn spread(ctx: Context<Spread>, sum: u64) -> Result<()> {
        let st = &mut ctx.accounts.tally;
        st.cycles += 1;

        let mut program = ctx.accounts.router_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            st.path_a += sum;
            program = ctx.remaining_accounts[0].clone();
        } else {
            st.path_b += sum;
        }

        let window = (st.cycles & 7) as u64 + 2;
        let base = sum / window + 1;
        let br = TallyBridge { a: ctx.accounts.a_meta.to_account_info(), b: ctx.accounts.b_meta.to_account_info(), c: ctx.accounts.c_meta.to_account_info() };
        let mut i = 0u64;
        while i < window {
            let amt = base + i;
            let cx = br.as_cpi(program.clone());
            br.cast(cx, amt)?;
            i += 1;
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Spread<'info> {
    #[account(init, payer = owner, space = 8 + 8 + 8 + 8)]
    pub tally: Account<'info, TallyState>,
    #[account(mut)] pub owner: Signer<'info>,
    /// CHECK:
    pub a_meta: AccountInfo<'info>,
    /// CHECK:
    pub b_meta: AccountInfo<'info>,
    /// CHECK:
    pub c_meta: AccountInfo<'info>,
    /// CHECK:
    pub router_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct TallyState { pub cycles: u64, pub path_a: u64, pub path_b: u64 }
#[derive(Clone)] pub struct TallyBridge<'info> { pub a: AccountInfo<'info>, pub b: AccountInfo<'info>, pub c: AccountInfo<'info> }
impl<'info> TallyBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_', 'info, TallyBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(*self.a.key, false),
            AccountMeta::new_readonly(*self.b.key, false),
            AccountMeta::new(*self.c.key, false),
        ]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.a.clone(), self.b.clone(), self.c.clone()] }
    pub fn cast(&self, ctx: CpiContext<'_, '_, '_, 'info, TallyBridge<'info>>, v: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: v.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
