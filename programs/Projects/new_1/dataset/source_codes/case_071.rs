use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWxSITEPROVWDRAW000000");

#[program]
pub mod site_provider_withdraw_insecure {
    use super::*;

    /// サイト提供者向け：累積された報酬トークンを引き出し、
    /// 残高をリセットします。署名チェックは一切ありません。
    pub fn withdraw_rewards(ctx: Context<WithdrawRewards>) {
        // ① 引き出し可能な累積報酬量
        let amount = ctx.accounts.provider_reward.total_awarded;

        // ② トークン転送（vault → provider_token_acc）
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to:   ctx.accounts.provider_token_acc.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let signer_seeds = &[&[b"vault_auth", &[*ctx.bumps.get("vault_authority").unwrap()]]];
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            ),
            amount,
        ).unwrap();

        // ③ 累積報酬をリセット
        ctx.accounts.provider_reward.total_awarded = 0;
    }
}

#[derive(Accounts)]
pub struct WithdrawRewards<'info> {
    /// サイト提供者アカウント（署名チェック omitted intentionally）
    pub provider:             AccountInfo<'info>,

    /// 報酬を蓄える Vault TokenAccount
    #[account(mut)]
    pub vault:                Account<'info, TokenAccount>,

    /// Vault の権限を持つ PDA（署名チェック omitted）
    #[account(
        seeds = [b"vault_auth"],
        bump
    )]
    pub vault_authority:      AccountInfo<'info>,

    /// 引き出し先のプロバイダ TokenAccount
    #[account(mut)]
    pub provider_token_acc:   Account<'info, TokenAccount>,

    /// 累積報酬を保持する PDA（事前に init 済み）
    #[account(mut)]
    pub provider_reward:      Account<'info, RewardData>,

    pub token_program:        Program<'info, Token>,
}

#[account]
pub struct RewardData {
    /// 累積付与トークン量
    pub total_awarded:  u64,
}
