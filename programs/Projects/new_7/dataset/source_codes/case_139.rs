// 6) spirit_summon_pad: 召喚リズム記録→外部効果反映（ループ→分岐→ループ違い）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
declare_id!("SpiritSum1111111111111111111111111111111");

#[program]
pub mod spirit_summon_pad {
    use super::*;
    pub fn ring(ctx: Context<Ring>, tone: u64) -> Result<()> {
        let s = &mut ctx.accounts.score;
        let mut next = ctx.accounts.effect_prog.to_account_info();

        for _ in 0..(tone % 2 + 1) { s.left += tone; }
        if s.left & 1 == 0 { s.right = s.right.wrapping_add(3); }
        for _ in 0..(tone % 3) { s.beat ^= Clock::get()?.slot; }
        if ctx.remaining_accounts.len() > 0 { next = ctx.remaining_accounts[0].clone(); s.routes += 1; }

        let br = EffectBridge { altar: ctx.accounts.altar.to_account_info(), speaker: ctx.accounts.speaker.to_account_info() };
        let cx = br.as_cpi(next.clone());
        br.fire(cx, tone.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ring<'info> {
    #[account(mut)]
    pub score: Account<'info, Rhythm>,
    /// CHECK:
    pub altar: AccountInfo<'info>,
    /// CHECK:
    pub speaker: AccountInfo<'info>,
    /// CHECK:
    pub effect_prog: AccountInfo<'info>,
}
#[account] pub struct Rhythm { pub left: u64, pub right: u64, pub beat: u64, pub routes: u64 }

#[derive(Clone)]
pub struct EffectBridge<'info> { pub altar: AccountInfo<'info>, pub speaker: AccountInfo<'info> }
impl<'info> EffectBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, EffectBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.altar.key, false), AccountMeta::new(*self.speaker.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.altar.clone(), self.speaker.clone()] }
    pub fn fire(&self, cx: CpiContext<'_, '_, '_, 'info, EffectBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
