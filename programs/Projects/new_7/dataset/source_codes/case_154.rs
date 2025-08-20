// 6) badge_queue_mixer: 先にループ→固定IDtransfer→分岐→動的CPI
// ループ→固定IDtransfer→分岐→動的CPI
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("BadgeQueueM11111111111111111111111111111");

#[program]
pub mod badge_queue_mixer {
    use super::*;
    pub fn settle(ctx: Context<Settle>, prize: u64, code: u64) -> Result<()> {
        for _ in 0..(code % 3 + 1) { ctx.accounts.meta.trace ^= code; }

        // 固定ID：transfer
        let t = Transfer {
            from: ctx.accounts.pool_token.to_account_info(),
            to: ctx.accounts.winner_token.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), t), prize)?;

        if prize > 0 { ctx.accounts.meta.counter += 1; }

        // 動的CPI：掲示
        let mut boardp = ctx.accounts.board_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 { boardp = ctx.remaining_accounts[0].clone(); ctx.accounts.meta.paths += 1; }
        let bb = BoardBridge { wall: ctx.accounts.board_wall.to_account_info(), user: ctx.accounts.winner_wallet.to_account_info() };
        bb.post(bb.as_cpi(boardp.clone()), code.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut)]
    pub meta: Account<'info, BadgeMeta>,

    // 固定ID
    #[account(mut)]
    pub pool_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    // 動的
    /// CHECK:
    pub board_wall: AccountInfo<'info>,
    /// CHECK:
    pub board_program: AccountInfo<'info>,
    /// CHECK:
    pub winner_wallet: AccountInfo<'info>,
}

#[account]
pub struct BadgeMeta { pub trace: u64, pub counter: u64, pub paths: u64 }

#[derive(Clone)]
pub struct BoardBridge<'info> { pub wall: AccountInfo<'info>, pub user: AccountInfo<'info> }
impl<'info> BoardBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, BoardBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.wall.key, false), AccountMeta::new_readonly(*self.user.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.wall.clone(), self.user.clone()] }
    pub fn post(&self, cx: CpiContext<'_, '_, '_, 'info, BoardBridge<'info>>, payload: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: payload };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
