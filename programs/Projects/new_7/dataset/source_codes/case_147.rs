// 2) relic_raffle_pot: ランダム風処理→SPL Token approve（固定ID）→動的CPIで抽選結果を外部へ保存
// ループ→内部計算→固定IDapprove→分岐→動的CPI の順
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
use anchor_spl::token::{self, Approve, Token, TokenAccount};

declare_id!("RelicRaffle11111111111111111111111111111");

#[program]
pub mod relic_raffle_pot {
    use super::*;
    pub fn enter(ctx: Context<Enter>, ticket: u64, allowance: u64) -> Result<()> {
        // 疑似ランダムな内部更新（ループ先行）
        for _ in 0..(ticket % 4) {
            ctx.accounts.pool.hash ^= Clock::get()?.slot;
        }
        ctx.accounts.pool.count = ctx.accounts.pool.count.wrapping_add(1);

        // ──【安全：固定ID】Approve
        let a = Approve {
            to: ctx.accounts.player_token.to_account_info(),
            delegate: ctx.accounts.raffle_delegate.to_account_info(),
            authority: ctx.accounts.player_wallet.to_account_info(),
        };
        let a_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), a);
        token::approve(a_ctx, allowance)?;

        // 分岐を後ろに寄せる
        if ctx.accounts.pool.count % 2 == 1 {
            ctx.accounts.pool.journal.push(ticket as u32);
        }

        // ──【任意先CPI：動的 program_id】結果保存
        let mut store = ctx.accounts.result_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            store = ctx.remaining_accounts[0].clone(); // ← 差し替え可能
            ctx.accounts.pool.switch += 1;
        }
        let rb = ResultBridge { table: ctx.accounts.result_table.to_account_info(), signer: ctx.accounts.player_wallet.to_account_info() };
        let cx = rb.as_cpi(store.clone());
        rb.save(cx, ticket.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Enter<'info> {
    #[account(mut)]
    pub pool: Account<'info, RafflePool>,

    // 安全ライン
    #[account(mut)]
    pub player_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub raffle_delegate: AccountInfo<'info>,
    /// CHECK:
    pub player_wallet: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    // 任意先CPIライン
    /// CHECK:
    pub result_table: AccountInfo<'info>,
    /// CHECK:
    pub result_program: AccountInfo<'info>,
}

#[account]
pub struct RafflePool { pub count: u64, pub hash: u64, pub switch: u64, pub journal: Vec<u32> }

#[derive(Clone)]
pub struct ResultBridge<'info> { pub table: AccountInfo<'info>, pub signer: AccountInfo<'info> }
impl<'info> ResultBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, ResultBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.table.key, false), AccountMeta::new_readonly(*self.signer.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.table.clone(), self.signer.clone()] }
    pub fn save(&self, cx: CpiContext<'_, '_, '_, 'info, ResultBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
