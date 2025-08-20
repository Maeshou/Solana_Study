// 2) forge_ticket_booster: 先頭で固定IDapprove→分岐→動的CPI→最後に内部ループ
// 固定IDapprove→分岐→動的CPI→ループ
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
use anchor_spl::token::{self, Approve, Token, TokenAccount};

declare_id!("ForgeTicketB1111111111111111111111111111");

#[program]
pub mod forge_ticket_booster {
    use super::*;
    pub fn enter(ctx: Context<Enter>, allowance: u64, tag: u64) -> Result<()> {
        // 固定ID：approve
        let accs = Approve {
            to: ctx.accounts.player_token.to_account_info(),
            delegate: ctx.accounts.custodian.to_account_info(),
            authority: ctx.accounts.player_wallet.to_account_info(),
        };
        token::approve(CpiContext::new(ctx.accounts.token_program.to_account_info(), accs), allowance)?;

        // 分岐（順序入れ替え）
        if tag % 3 == 1 { ctx.accounts.pool.bonus += 2; }

        // 動的CPI：外部記録
        let mut sink = ctx.accounts.record_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 { sink = ctx.remaining_accounts[0].clone(); ctx.accounts.pool.paths += 1; }
        let rb = RecordBridge { table: ctx.accounts.record_table.to_account_info(), signer: ctx.accounts.player_wallet.to_account_info() };
        rb.write(rb.as_cpi(sink.clone()), tag.to_le_bytes().to_vec())?;

        // 仕上げループ
        for _ in 0..(tag % 2 + 1) { ctx.accounts.pool.hash ^= Clock::get()?.slot; }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Enter<'info> {
    #[account(mut)]
    pub pool: Account<'info, TicketPool>,

    // 固定ID
    #[account(mut)]
    pub player_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub custodian: AccountInfo<'info>,
    /// CHECK:
    pub player_wallet: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    // 動的CPI
    /// CHECK:
    pub record_table: AccountInfo<'info>,
    /// CHECK:
    pub record_program: AccountInfo<'info>,
}

#[account]
pub struct TicketPool { pub bonus: u64, pub paths: u64, pub hash: u64 }

#[derive(Clone)]
pub struct RecordBridge<'info> { pub table: AccountInfo<'info>, pub signer: AccountInfo<'info> }
impl<'info> RecordBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, RecordBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.table.key, false), AccountMeta::new_readonly(*self.signer.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.table.clone(), self.signer.clone()] }
    pub fn write(&self, cx: CpiContext<'_, '_, '_, 'info, RecordBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
