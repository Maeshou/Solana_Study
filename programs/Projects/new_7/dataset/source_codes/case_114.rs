// 3) tiered_redeemer: レベルでデータにヘッダを付ける（2要素u64を連結）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("TierRedeem111111111111111111111111111111");

#[program]
pub mod tiered_redeemer {
    use super::*;
    pub fn redeem(ctx: Context<Redeem>, qty: u64) -> Result<()> {
        let st = &mut ctx.accounts.tier_state;
        st.level += 1;

        let mut program = ctx.accounts.bridge_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            st.sent_a += qty;
            program = ctx.remaining_accounts[0].clone();
        } else {
            st.sent_b += qty;
        }

        let br = TierBridge { src: ctx.accounts.ticket_buf.to_account_info(), dst: ctx.accounts.reward_buf.to_account_info() };
        let payload = build_payload(st.level as u64, qty);
        let cx = br.as_cpi(program.clone());
        br.emit(cx, payload)?;
        Ok(())
    }
    fn build_payload(h: u64, v: u64) -> Vec<u8> {
        let mut out = h.to_le_bytes().to_vec();
        out.extend_from_slice(&v.to_le_bytes());
        out
    }
}
#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 8 + 8)]
    pub tier_state: Account<'info, TierState>,
    #[account(mut)] pub payer: Signer<'info>,
    /// CHECK:
    pub ticket_buf: AccountInfo<'info>,
    /// CHECK:
    pub reward_buf: AccountInfo<'info>,
    /// CHECK:
    pub bridge_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct TierState { pub level: u64, pub sent_a: u64, pub sent_b: u64 }
#[derive(Clone)] pub struct TierBridge<'info> { pub src: AccountInfo<'info>, pub dst: AccountInfo<'info> }
impl<'info> TierBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, TierBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new_readonly(*self.src.key, false), AccountMeta::new(*self.dst.key, false)] }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.src.clone(), self.dst.clone()] }
    pub fn emit(&self, ctx: CpiContext<'_, '_, '_, 'info, TierBridge<'info>>, bytes: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: bytes };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
