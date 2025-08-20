// 11) guild_treasure_share: ギルド宝物庫から寄付を複数メンバーに配分
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Gu1ldTre4sureSh4reBBBBBBBBBBBBBBBBBBBBBBB");

#[program]
pub mod guild_treasure_share {
    use super::*;

    pub fn setup_guild(
        ctx: Context<SetupGuild>,
        initial_fund: u64,
        seed: u64,
    ) -> Result<()> {
        let guild = &mut ctx.accounts.guild_state;
        guild.leader = ctx.accounts.leader.key();
        guild.total_fund = initial_fund.saturating_add(seed % 10 + 1); // 計算で初期化
        guild.share_count = (seed / 2) % 5 + 2;
        guild.history_index = initial_fund / 3 + 1;
        Ok(())
    }

    pub fn distribute_share(
        ctx: Context<DistributeShare>,
        members: u8,
        contribution_factor: u64,
    ) -> Result<()> {
        let guild = &mut ctx.accounts.guild_state;

        // 基本配分額の計算（最低1にする）
        let mut base_amount = contribution_factor.saturating_mul(guild.share_count as u64);
        if base_amount < 1 {
            base_amount = guild.share_count as u64 + 1;
        }

        // メンバー数に応じた総額
        let mut total_amount = base_amount.saturating_mul(members as u64);
        let mut round_counter = 0;

        // 各メンバーへ順番に配分（簡易ループ）
        while round_counter < members {
            let portion = (total_amount / (members as u64)).saturating_add(1);

            let ix = token_ix::transfer(
                &ctx.accounts.token_program.key(),
                &ctx.accounts.guild_vault.key(),
                &ctx.accounts.member_vault.key(),
                &ctx.accounts.leader.key(),
                &[],
                portion,
            )?;
            invoke(
                &ix,
                &[
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.guild_vault.to_account_info(),
                    ctx.accounts.member_vault.to_account_info(),
                    ctx.accounts.leader.to_account_info(),
                ],
            )?;

            total_amount = total_amount.saturating_sub(portion);
            round_counter = round_counter.saturating_add(1);
        }

        // 余剰があればまとめて送付
        if total_amount > 0 {
            let ix2 = token_ix::transfer(
                &ctx.accounts.token_program.key(),
                &ctx.accounts.guild_vault.key(),
                &ctx.accounts.member_vault.key(),
                &ctx.accounts.leader.key(),
                &[],
                total_amount,
            )?;
            invoke(
                &ix2,
                &[
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.guild_vault.to_account_info(),
                    ctx.accounts.member_vault.to_account_info(),
                    ctx.accounts.leader.to_account_info(),
                ],
            )?;
        }

        // 履歴と統計の更新
        guild.total_fund = guild.total_fund.saturating_sub(base_amount);
        guild.history_index = guild.history_index.saturating_add(1);
        guild.share_count = guild.share_count.saturating_add(1);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupGuild<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 8 + 8 + 8)]
    pub guild_state: Account<'info, GuildState>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeShare<'info> {
    #[account(mut, has_one = leader)]
    pub guild_state: Account<'info, GuildState>,
    pub leader: Signer<'info>,
    #[account(mut)]
    pub guild_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub member_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct GuildState {
    pub leader: Pubkey,
    pub total_fund: u64,
    pub share_count: u64,
    pub history_index: u64,
}
