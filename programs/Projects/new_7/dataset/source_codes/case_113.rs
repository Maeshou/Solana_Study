// 4) mosaic_merger: 固定長フラグメントを先頭ゼロ詰めし連結（符号化の多様化）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("MosaicMerge111111111111111111111111111111");

#[program]
pub mod mosaic_merger {
    use super::*;
    pub fn merge(ctx: Context<Merge>, tiles: u64) -> Result<()> {
        let st = &mut ctx.accounts.merge_state;
        st.round += 1;

        let mut program = ctx.accounts.pipe_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            st.path_a += tiles;
            program = ctx.remaining_accounts[0].clone();
        } else {
            st.path_b += tiles;
        }

        let mut payload = vec![0u8; 16];
        payload[0..8].copy_from_slice(&(tiles + st.round).to_le_bytes());
        payload[8..16].copy_from_slice(&st.round.to_le_bytes());

        let br = MergeBridge { a: ctx.accounts.input_buf.to_account_info(), b: ctx.accounts.output_buf.to_account_info() };
        let cx = br.as_cpi(program.clone());
        br.transfer(cx, payload)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Merge<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 8)]
    pub merge_state: Account<'info, MergeState>,
    #[account(mut)] pub user: Signer<'info>,
    /// CHECK:
    pub input_buf: AccountInfo<'info>,
    /// CHECK:
    pub output_buf: AccountInfo<'info>,
    /// CHECK:
    pub pipe_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct MergeState { pub round: u64, pub path_a: u64, pub path_b: u64 }
#[derive(Clone)] pub struct MergeBridge<'info> { pub a: AccountInfo<'info>, pub b: AccountInfo<'info> }
impl<'info> MergeBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, MergeBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.a.key, false), AccountMeta::new(*self.b.key, false)] }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.a.clone(), self.b.clone()] }
    pub fn transfer(&self, ctx: CpiContext<'_, '_, '_, 'info, MergeBridge<'info>>, bytes: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: bytes };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
