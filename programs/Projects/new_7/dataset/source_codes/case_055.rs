// 1) guild_bonus_payout: ギルドの功績に応じてボーナス送付（計算→記録→送付）
use anchor_lang::prelude::*;
use solana_program::{program::invoke, instruction::Instruction};
use spl_token::instruction as token_ix;

declare_id!("Gu1ldPay111111111111111111111111111111111");

#[program]
pub mod guild_bonus_payout {
    use super::*;
    pub fn init(ctx: Context<Init>, name: String, fee_bps: u16) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.admin = ctx.accounts.admin.key();
        g.name = name;
        g.fee_bps = fee_bps.min(800);
        g.paid_total = 0;
        g.round = 0;
        Ok(())
    }

    pub fn pay_bonus(ctx: Context<PayBonus>, points: u64, round_increase: u8) -> Result<()> {
        // 係数づくり（簡単な while ループ）
        let mut k = 9u64;
        let mut r = 0;
        while r < round_increase {
            k = k.saturating_add(2);
            r += 1;
        }

        // 分岐：ポイントが少ない場合はメモ更新のみ
        let gross = points.saturating_mul(k);
        let fee = gross.saturating_mul(ctx.accounts.guild.fee_bps as u64) / 10_000;
        let net = if gross > fee { gross - fee } else { 0 };

        if net == 0 {
            // ラウンドだけ進め、ログを追記
            ctx.accounts.guild.round = ctx.accounts.guild.round.saturating_add(1);
            ctx.accounts.memo_log = ctx.accounts.memo_log.saturating_add(1);
            return Ok(());
        } else {
            ctx.accounts.guild.paid_total = ctx.accounts.guild.paid_total.saturating_add(net);
        }

        // 手動で Instruction を組み立て → 実行
        let ix = token_ix::transfer(
            &ctx.accounts.any_token_program.key(), // ← 外部から渡された program id を使用
            &ctx.accounts.treasury.key(),
            &ctx.accounts.member_vault.key(),
            &ctx.accounts.admin.key(),
            &[],
            net,
        )?;

        invoke(
            &ix,
            &[
                ctx.accounts.any_token_program.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.member_vault.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
        )?;

        // 後処理：メモ用の数値を少しずつ増やす
        let mut i = 0;
        while i < 3 {
            ctx.accounts.memo_log = ctx.accounts.memo_log.saturating_add(i as u64 + 1);
            i += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 64 + 2 + 8 + 8)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayBonus<'info> {
    #[account(mut, has_one = admin)]
    pub guild: Account<'info, Guild>,
    pub admin: Signer<'info>,
    /// CHECK: 外部入力
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_vault: UncheckedAccount<'info>,
    /// CHECK: Program アカウントを直接受ける
    pub any_token_program: UncheckedAccount<'info>,
    /// CHECK: 簡易ログ
    #[account(mut)]
    pub memo_holder: UncheckedAccount<'info>,
    // メモは単なる数値として保持
    // storage: memo_holder.data.borrow_mut() を避けるため、Guild に累積
}

#[account]
pub struct Guild {
    pub admin: Pubkey,
    pub name: String,
    pub fee_bps: u16,
    pub paid_total: u64,
    pub round: u64,
    pub memo_log: u64,
}
