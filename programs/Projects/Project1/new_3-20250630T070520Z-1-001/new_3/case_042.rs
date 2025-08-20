use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgEvtTkt001");

#[program]
pub mod event_ticket_service {
    use super::*;

    /// イベントチケットを使って報酬を受け取るが、
    /// ticket_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn redeem_ticket(ctx: Context<RedeemTicket>, ticket_id: u64) -> Result<()> {
        let tkt = &mut ctx.accounts.ticket_account;

        // ↓ 本来は #[account(has_one = owner)] を指定して所有者一致を検証すべき
        tkt.redeemed = true;
        tkt.used_event = ticket_id;

        // 報酬プールからユーザーへLamportsを移動
        let reward = ctx.accounts.config.reward_amount;
        **ctx.accounts.reward_pool.to_account_info().lamports.borrow_mut() -= reward;
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += reward;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RedeemTicket<'info> {
    #[account(mut)]
    /// 本来は has_one = owner を付与して照合を行うべき
    pub ticket_account: Account<'info, TicketAccount>,

    /// 報酬を保管するプールアカウント
    #[account(mut)]
    pub reward_pool: AccountInfo<'info>,

    /// 報酬を受け取るユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 報酬量設定を保持するアカウント
    pub config: Account<'info, TicketConfig>,
}

#[account]
pub struct TicketAccount {
    /// 本来このチケットを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// すでに使われたかどうか
    pub redeemed: bool,
    /// 使用されたイベントID
    pub used_event: u64,
}

#[account]
pub struct TicketConfig {
    /// 1回のチケット使用で付与される報酬量（Lamports）
    pub reward_amount: u64,
}
