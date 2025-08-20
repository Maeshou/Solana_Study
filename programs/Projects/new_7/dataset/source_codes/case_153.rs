// 5) relic_rank_panel: 動的CPI→固定IDburn→内部計算→最後にループ
// 動的CPI→固定IDburn→内部計算→ループ
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Burn, Token, TokenAccount, Mint};

declare_id!("RelicRankPa11111111111111111111111111111");

#[program]
pub mod relic_rank_panel {
    use super::*;
    pub fn update(ctx: Context<Update>, rank_tag: u64, burn_amt: u64) -> Result<()> {
        // 動的CPI：外部ランキング
        let mut rp = ctx.accounts.rank_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 { rp = ctx.remaining_accounts[0].clone(); ctx.accounts.rank_meta.paths += 1; }
        let rb = RankBridge { board: ctx.accounts.rank_board.to_account_info(), signer: ctx.accounts.referee_wallet.to_account_info() };
        rb.push(rb.as_cpi(rp.clone()), rank_tag.to_le_bytes().to_vec())?;

        // 固定ID：burn
        let b = Burn {
            mint: ctx.accounts.reward_mint.to_account_info(),
            from: ctx.accounts.reward_token.to_account_info(),
            authority: ctx.accounts.reward_authority.to_account_info(),
        };
        token::burn(CpiContext::new(ctx.accounts.token_program.to_account_info(), b), burn_amt)?;

        // 内部計算
        ctx.accounts.rank_meta.hash ^= Clock::get()?.slot;

        // ループ
        for _ in 0..(rank_tag % 2 + 1) { ctx.accounts.rank_meta.ticks = ctx.accounts.rank_meta.ticks.wrapping_add(1); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub rank_meta: Account<'info, RankMeta>,

    // 動的
    /// CHECK:
    pub rank_board: AccountInfo<'info>,
    /// CHECK:
    pub rank_program: AccountInfo<'info>,
    /// CHECK:
    pub referee_wallet: AccountInfo<'info>,

    // 固定ID
    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub reward_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub reward_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RankMeta { pub paths: u64, pub hash: u64, pub ticks: u64 }

#[derive(Clone)]
pub struct RankBridge<'info> { pub board: AccountInfo<'info>, pub signer: AccountInfo<'info> }
impl<'info> RankBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, RankBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.board.key, false), AccountMeta::new_readonly(*self.signer.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.board.clone(), self.signer.clone()] }
    pub fn push(&self, cx: CpiContext<'_, '_, '_, 'info, RankBridge<'info>>, payload: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: payload };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
