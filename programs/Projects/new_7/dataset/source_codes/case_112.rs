// 5) beacon_pulse: 周波数を slot と合成、奇数パルスだけ送信
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("BeaconPulse11111111111111111111111111111");

#[program]
pub mod beacon_pulse {
    use super::*;
    pub fn ping(ctx: Context<Ping>, ticks: u64) -> Result<()> {
        let st = &mut ctx.accounts.beacon;
        st.counter += 1;

        let mut program = ctx.accounts.emit_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            st.path_a += ticks;
            program = ctx.remaining_accounts[0].clone();
        } else {
            st.path_b += ticks;
        }

        let init = (Clock::get()?.slot & 15) as u64 + 1;
        let mut i = 0u64;
        let br = BeaconBridge { tower: ctx.accounts.tower_buf.to_account_info(), ether: ctx.accounts.ether_buf.to_account_info() };
        while i < ticks {
            let freq = init + (i & 3);
            if (freq & 1) > 0 {
                let cx = br.as_cpi(program.clone());
                br.pulse(cx, freq + st.counter)?;
            }
            i += 1;
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Ping<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 8 + 8)]
    pub beacon: Account<'info, BeaconState>,
    #[account(mut)] pub payer: Signer<'info>,
    /// CHECK:
    pub tower_buf: AccountInfo<'info>,
    /// CHECK:
    pub ether_buf: AccountInfo<'info>,
    /// CHECK:
    pub emit_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct BeaconState { pub counter: u64, pub path_a: u64, pub path_b: u64 }
#[derive(Clone)] pub struct BeaconBridge<'info> { pub tower: AccountInfo<'info>, pub ether: AccountInfo<'info> }
impl<'info> BeaconBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, BeaconBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new_readonly(*self.tower.key, false), AccountMeta::new(*self.ether.key, false)] }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.tower.clone(), self.ether.clone()] }
    pub fn pulse(&self, ctx: CpiContext<'_, '_, '_, 'info, BeaconBridge<'info>>, v: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: v.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
