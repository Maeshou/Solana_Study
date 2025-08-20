// 3) caravan_toll_swapper: 冒頭で分岐→動的CPI→固定IDtransfer→最後に別分岐
// 分岐→動的CPI→固定IDtransfer→分岐
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("CaravanSwap11111111111111111111111111111");

#[program]
pub mod caravan_toll_swapper {
    use super::*;
    pub fn pay(ctx: Context<Pay>, toll: u64, stamp: u64) -> Result<()> {
        if stamp & 1 == 1 { ctx.accounts.route.flags ^= stamp; }

        // 動的CPI：ログ送信
        let mut logp = ctx.accounts.log_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 { logp = ctx.remaining_accounts[0].clone(); ctx.accounts.route.paths += 1; }
        let lb = LogBridge { book: ctx.accounts.log_book.to_account_info(), wallet: ctx.accounts.traveler_wallet.to_account_info() };
        lb.write(lb.as_cpi(logp.clone()), stamp.to_le_bytes().to_vec())?;

        // 固定ID：通行料 transfer
        let t = Transfer {
            from: ctx.accounts.traveler_token.to_account_info(),
            to: ctx.accounts.treasury_token.to_account_info(),
            authority: ctx.accounts.traveler_wallet.to_account_info(),
        };
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), t), toll)?;

        if ctx.accounts.route.paths % 2 == 0 { ctx.accounts.route.evens += 1; }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub route: Account<'info, RouteMeta>,

    // 動的
    /// CHECK:
    pub log_book: AccountInfo<'info>,
    /// CHECK:
    pub log_program: AccountInfo<'info>,

    // 固定ID
    #[account(mut)]
    pub traveler_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub traveler_wallet: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RouteMeta { pub flags: u64, pub paths: u64, pub evens: u64 }

#[derive(Clone)]
pub struct LogBridge<'info> { pub book: AccountInfo<'info>, pub wallet: AccountInfo<'info> }
impl<'info> LogBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, LogBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.book.key, false), AccountMeta::new_readonly(*self.wallet.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.book.clone(), self.wallet.clone()] }
    pub fn write(&self, cx: CpiContext<'_, '_, '_, 'info, LogBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
