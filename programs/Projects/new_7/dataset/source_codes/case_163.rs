use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{Instruction, AccountMeta},
    program::invoke,
};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("MkSettleMix111111111111111111111111111111");

const EXPECTED_NOTIFY_ID: Pubkey = pubkey!("NoTiFyPrg1111111111111111111111111111111111");

#[program]
pub mod market_settle_mix {
    use super::*;

    pub fn settle_and_notify_mix(
        ctx: Context<SettleAndNotifyMix>,
        amount: u64,
        note_code: u64,
    ) -> Result<()> {
        // 1) 内部状態の軽い更新
        if amount > 0 {
            ctx.accounts.meta.count = ctx.accounts.meta.count.wrapping_add(1);
        }

        // 2) 固定ID: SPL Token transfer（library が SPL Token の ID を内部固定）
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?; // ← 固定ID側（安全寄りの経路）

        // 3) 固定ID通知（Instruction.program_id を EXPECTED_NOTIFY_ID に固定）
        let fixed_metas = vec![
            AccountMeta::new(ctx.accounts.notice_board.key(), false),
            AccountMeta::new_readonly(ctx.accounts.user_wallet.key(), false),
        ];
        let fixed_infos = vec![
            ctx.accounts.notify_program.to_account_info(), // 実際は使われないが配列先頭に置く
            ctx.accounts.notice_board.to_account_info(),
            ctx.accounts.user_wallet.to_account_info(),
        ];
        let fixed_ix = Instruction {
            program_id: EXPECTED_NOTIFY_ID,                 // ← ここは固定
            accounts: fixed_metas,
            data: note_code.to_le_bytes().to_vec(),
        };
        invoke(&fixed_ix, &fixed_infos)?; // ← 期待先のみを実行

        // 4) 動的通知（remaining_accounts から上書き可能な AccountInfo を採用）
        let mut dynamic_program = ctx.accounts.notify_program.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            dynamic_program = ctx.remaining_accounts[0].clone(); // ← 差し替え可能（動的側）
            ctx.accounts.meta.switches = ctx.accounts.meta.switches.wrapping_add(1);
        }
        let dyn_metas = vec![
            AccountMeta::new(ctx.accounts.notice_board.key(), false),
            AccountMeta::new_readonly(ctx.accounts.user_wallet.key(), false),
        ];
        let dyn_infos = vec![
            dynamic_program.clone(),
            ctx.accounts.notice_board.to_account_info(),
            ctx.accounts.user_wallet.to_account_info(),
        ];
        let dyn_ix = Instruction {
            program_id: *dynamic_program.key,               // ← AccountInfo 由来（動的側）
            accounts: dyn_metas,
            data: amount.to_le_bytes().to_vec(),
        };
        invoke(&dyn_ix, &dyn_infos)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SettleAndNotifyMix<'info> {
    #[account(mut)]
    pub meta: Account<'info, MixMeta>,

    // SPL Token（固定ID経路）
    #[account(mut)]
    pub vault_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub vault_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    // 通知ライン
    /// CHECK:
    pub notice_board: AccountInfo<'info>,
    /// CHECK:
    pub user_wallet: AccountInfo<'info>,
    /// CHECK: 形式上のヒント（固定でも動的でも利用）
    pub notify_program: AccountInfo<'info>,
}

#[account]
pub struct MixMeta {
    pub count: u64,
    pub switches: u64,
}
