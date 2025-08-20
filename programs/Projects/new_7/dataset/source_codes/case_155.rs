// 7) quest_sync_hub: 最初に動的CPI→分岐→固定IDrevoke→最後にループ
// 動的CPI→分岐→固定IDrevoke→ループ
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Revoke, Token, TokenAccount};

declare_id!("QuestSyncHb11111111111111111111111111111");

#[program]
pub mod quest_sync_hub {
    use super::*;
    pub fn sync(ctx: Context<Sync>, step: u64) -> Result<()> {
        // 動的CPI：外部同期
        let mut sp = ctx.accounts.sync_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 { sp = ctx.remaining_accounts[0].clone(); ctx.accounts.log.paths += 1; }
        let sb = SyncBridge { book: ctx.accounts.sync_book.to_account_info(), user: ctx.accounts.player_wallet.to_account_info() };
        sb.push(sb.as_cpi(sp.clone()), step.to_le_bytes().to_vec())?;

        if step % 2 == 0 { ctx.accounts.log.evens += 1; }

        // 固定ID：revoke（委任解除）
        let r = Revoke { source: ctx.accounts.player_token.to_account_info(), authority: ctx.accounts.player_wallet.to_account_info() };
        token::revoke(CpiContext::new(ctx.accounts.token_program.to_account_info(), r))?;

        for _ in 0..(step % 3 + 1) { ctx.accounts.log.hash ^= Clock::get()?.slot; }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Sync<'info> {
    #[account(mut)]
    pub log: Account<'info, SyncLog>,

    // 動的
    /// CHECK:
    pub sync_book: AccountInfo<'info>,
    /// CHECK:
    pub sync_program: AccountInfo<'info>,
    /// CHECK:
    pub player_wallet: AccountInfo<'info>,

    // 固定ID
    #[account(mut)]
    pub player_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct SyncLog { pub paths: u64, pub evens: u64, pub hash: u64 }

#[derive(Clone)]
pub struct SyncBridge<'info> { pub book: AccountInfo<'info>, pub user: AccountInfo<'info> }
impl<'info> SyncBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, SyncBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.book.key, false), AccountMeta::new_readonly(*self.user.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.book.clone(), self.user.clone()] }
    pub fn push(&self, cx: CpiContext<'_, '_, '_, 'info, SyncBridge<'info>>, payload: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: payload };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
