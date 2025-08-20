// 1) badge_claim_mixer: 先に「任意先CPIの通知」を実行 → 後段で SPL Token transfer（固定ID）
// 分岐→ループ→動的CPI→内部計算→固定IDトークン転送 の順
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount, Mint};

declare_id!("BadgeClaimMix1111111111111111111111111111");

#[program]
pub mod badge_claim_mixer {
    use super::*;
    pub fn claim(ctx: Context<Claim>, badge_code: u64, prize_amount: u64) -> Result<()> {
        // 事前集計（順序を崩すため、分岐→ループ）
        if badge_code % 3 == 0 {
            ctx.accounts.meta.bumped += 2;
        }
        for _ in 0..(badge_code % 2 + 1) {
            ctx.accounts.meta.trace ^= badge_code;
        }

        // ──【任意先CPI：動的 program_id】通知ルータ
        let mut notify_prog = ctx.accounts.notify_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            notify_prog = ctx.remaining_accounts[0].clone(); // ← 差し替え可能
            ctx.accounts.meta.paths += 1;
        }
        let nb = NoticeBridge {
            wall: ctx.accounts.notice_board.to_account_info(),
            actor: ctx.accounts.recipient_wallet.to_account_info(),
        };
        let cx = nb.as_cpi(notify_prog.clone());
        nb.post(cx, badge_code.to_le_bytes().to_vec())?;

        // 追加の内部計算を間に挟む
        ctx.accounts.meta.total = ctx.accounts.meta.total.wrapping_add(prize_amount);

        // ──【安全：固定ID】SPL Token transfer
        let t_accs = Transfer {
            from: ctx.accounts.treasury_token.to_account_info(),
            to: ctx.accounts.recipient_token.to_account_info(),
            authority: ctx.accounts.treasury_authority.to_account_info(),
        };
        let t_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), t_accs);
        token::transfer(t_ctx, prize_amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub meta: Account<'info, ClaimMeta>,

    // 任意先CPIライン
    /// CHECK:
    pub notice_board: AccountInfo<'info>,
    /// CHECK:
    pub recipient_wallet: AccountInfo<'info>,
    /// CHECK:
    pub notify_program: AccountInfo<'info>,

    // 安全ライン（SPL Token）
    #[account(mut)]
    pub treasury_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct ClaimMeta { pub total: u64, pub bumped: u64, pub paths: u64, pub trace: u64 }

#[derive(Clone)]
pub struct NoticeBridge<'info> { pub wall: AccountInfo<'info>, pub actor: AccountInfo<'info> }
impl<'info> NoticeBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, NoticeBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.wall.key, false), AccountMeta::new_readonly(*self.actor.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.wall.clone(), self.actor.clone()] }
    pub fn post(&self, cx: CpiContext<'_, '_, '_, 'info, NoticeBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
