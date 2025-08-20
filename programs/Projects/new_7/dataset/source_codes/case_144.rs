use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount, Mint};

declare_id!("MkPlaceFixTokenButDynNotify1111111111111");

#[program]
pub mod market_payout_and_notify {
    use super::*;
    pub fn settle_and_notify(
        ctx: Context<SettleAndNotify>,
        amount: u64,
        note_code: u64,
    ) -> Result<()> {
        // ──【安全ライン】SPL Token 送金：program_id は spl_token::ID に固定
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.winner_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?; // ← ここは固定IDで安全

        // ──【危険ライン】通知ルータ（任意先へCPI可能）：program_id を動的採用
        let mut router = ctx.accounts.notify_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            router = ctx.remaining_accounts[0].clone(); // ← 差し替え可能
            ctx.accounts.settlement_meta.switched += 1;
        }

        let br = NotifyBridge {
            board: ctx.accounts.notice_board.to_account_info(),
            actor: ctx.accounts.winner_wallet.to_account_info(),
        };
        let payload = note_code.to_le_bytes().to_vec();
        let cx = br.as_cpi(router.clone());
        br.post(cx, payload)?; // ← program_id = *cx.program.key（動的）
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SettleAndNotify<'info> {
    #[account(mut)]
    pub settlement_meta: Account<'info, SettlementMeta>,

    // SPL Token 安全ライン
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_token_account: Account<'info, TokenAccount>,
    /// CHECK: 権限者（署名想定）。実際は seeds などで厳格化を推奨
    pub vault_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    // 動的CPIライン（脆弱）
    /// CHECK:
    pub notice_board: AccountInfo<'info>,
    /// CHECK:
    pub winner_wallet: AccountInfo<'info>,
    /// CHECK:
    pub notify_program: AccountInfo<'info>,
}

#[account]
pub struct SettlementMeta {
    pub switched: u64,
}

#[derive(Clone)]
pub struct NotifyBridge<'info> {
    pub board: AccountInfo<'info>,
    pub actor: AccountInfo<'info>,
}
impl<'info> NotifyBridge<'info> {
    pub fn as_cpi(
        &self,
        p: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, NotifyBridge<'info>> {
        CpiContext::new(p, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(*self.board.key, false),
            AccountMeta::new_readonly(*self.actor.key, false),
        ]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![p.clone(), self.board.clone(), self.actor.clone()]
    }
    pub fn post(
        &self,
        cx: CpiContext<'_, '_, '_, 'info, NotifyBridge<'info>>,
        data: Vec<u8>,
    ) -> Result<()> {
        let ix = Instruction {
            program_id: *cx.program.key, // ← 動的
            accounts: self.metas(),
            data,
        };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
