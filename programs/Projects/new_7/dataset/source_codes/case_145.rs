use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Approve, Revoke, Token, TokenAccount};

declare_id!("ForgeFixTokenButDynLog111111111111111111");

#[program]
pub mod forge_boost_and_log {
    use super::*;
    pub fn boost_then_log(ctx: Context<BoostThenLog>, allowance: u64, log_tag: u64) -> Result<()> {
        // ──【安全ライン】SPL Token approve
        let approve_accs = Approve {
            to: ctx.accounts.player_token.to_account_info(),
            delegate: ctx.accounts.boost_delegate.to_account_info(),
            authority: ctx.accounts.player_wallet.to_account_info(),
        };
        let approve_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), approve_accs);
        token::approve(approve_ctx, allowance)?; // ← 固定ID

        // 適当な内部計算
        ctx.accounts.boost_state.gauge ^= allowance;

        // ──【危険ライン】外部ログ：program_id 動的
        let mut logger = ctx.accounts.log_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            logger = ctx.remaining_accounts[0].clone(); // ← 差し替え可能
            ctx.accounts.boost_state.routes += 1;
        }
        let lb = LogBridge {
            book: ctx.accounts.log_book.to_account_info(),
            owner: ctx.accounts.player_wallet.to_account_info(),
        };
        let payload = log_tag.to_le_bytes().to_vec();
        let cx = lb.as_cpi(logger.clone());
        lb.write(cx, payload)?; // ← 動的CPI

        // ──【安全ライン】SPL Token revoke
        let revoke_accs = Revoke {
            source: ctx.accounts.player_token.to_account_info(),
            authority: ctx.accounts.player_wallet.to_account_info(),
        };
        let revoke_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), revoke_accs);
        token::revoke(revoke_ctx)?; // ← 固定ID
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BoostThenLog<'info> {
    #[account(mut)]
    pub boost_state: Account<'info, BoostState>,

    // SPL Token 安全ライン
    #[account(mut)]
    pub player_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub boost_delegate: AccountInfo<'info>,
    /// CHECK:
    pub player_wallet: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    // 動的CPIライン
    /// CHECK:
    pub log_book: AccountInfo<'info>,
    /// CHECK:
    pub log_program: AccountInfo<'info>,
}

#[account]
pub struct BoostState {
    pub gauge: u64,
    pub routes: u64,
}

#[derive(Clone)]
pub struct LogBridge<'info> {
    pub book: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
}
impl<'info> LogBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, LogBridge<'info>> {
        CpiContext::new(p, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(*self.book.key, false),
            AccountMeta::new_readonly(*self.owner.key, false),
        ]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![p.clone(), self.book.clone(), self.owner.clone()]
    }
    pub fn write(
        &self,
        cx: CpiContext<'_, '_, '_, 'info, LogBridge<'info>>,
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
