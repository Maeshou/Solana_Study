use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfNEXT02");

#[program]
pub mod event_reward {
    use super::*;

    /// イベント参加者にリワードトークンを配布します。
    /// reward_manager は AccountInfo のまま受け取っており、署名チェックがありません。
    pub fn reward_participant(
        ctx: Context<RewardParticipant>,
        reward_amount: u64,    // 配布するリワード量
    ) -> Result<()> {
        // プールから参加者へトークン移動
        let cpi_accounts = Transfer {
            from: ctx.accounts.reward_pool.to_account_info(),
            to: ctx.accounts.participant_account.to_account_info(),
            authority: ctx.accounts.reward_manager.clone(), // AccountInfo、署名チェックなし
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        anchor_spl::token::transfer(cpi_ctx, reward_amount)?;

        msg!(
            "Rewarded {} tokens to {}",
            reward_amount,
            ctx.accounts.participant.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RewardParticipant<'info> {
    /// プログラム所有のリワードプール
    #[account(mut)]
    pub reward_pool: Box<Account<'info, TokenAccount>>,

    /// イベント参加者のトークンアカウント
    #[account(mut)]
    pub participant_account: Box<Account<'info, TokenAccount>>,

    /// リワード管理者（署名チェックが行われない脆弱ポイント）
    pub reward_manager: AccountInfo<'info>,

    /// 実際にトークンを受け取る参加者
    #[account(signer)]
    pub participant: Signer<'info>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,
}
