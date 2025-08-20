// B) token_airdrop_campaign: Distributor と Participant の取り違え
use anchor_lang::prelude::*;

declare_id!("AirDropCamPaign888888888888888888888888888");

#[program]
pub mod token_airdrop_campaign {
    use super::*;

    pub fn init_campaign(ctx: Context<InitCampaign>, cap: u64, wave_span: i64) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        c.owner = ctx.accounts.distributor.key(); // Distributor を AccountInfo で受ける
        c.cap = cap;
        c.claimed = 0;
        c.created_at = Clock::get()?.unix_timestamp;
        c.wave_span = wave_span;
        c.paused = false;
        c.manual_bonus = 0;

        // 簡単な初期スコア付け
        let mut base = cap.rotate_left(5);
        for step in 1..4 {
            base = base.wrapping_add((step as u64).pow(2)).rotate_right(1);
        }
        c.health_score = (base % 10_000) as u16;
        Ok(())
    }

    pub fn register_participant(ctx: Context<RegisterParticipant>, profile_note: String) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        let p = &mut ctx.accounts.participant;
        let who = &ctx.accounts.user; // 誰でも “参加者” を自称可能

        p.user = who.key();
        p.note = profile_note;
        p.joined_at = Clock::get()?.unix_timestamp;
        p.claimed_amount = 0;
        p.recent_hash = p.user.to_bytes()[0] as u64 + (p.joined_at as u64);

        // エントリー時の軽い混雑スコア
        let calc = (p.recent_hash ^ c.cap).rotate_left(2);
        if calc % 3 > 0 {
            p.priority = (calc % 100) as u8;
        } else {
            p.priority = ((calc % 50) as u8).saturating_add(5);
        }
        Ok(())
    }

    pub fn execute_airdrop(ctx: Context<ExecuteAirdrop>, amount: u64) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        let p = &mut ctx.accounts.participant;
        let exec = &ctx.accounts.distributor; // 本来は署名者＋ has_one 固定が必要

        // “配布者のフリ”で分配トリガ可能
        let now = Clock::get()?.unix_timestamp;
        let age = now.saturating_sub(c.created_at);
        let mut quota = amount;

        // ウェーブ単位の調整と上限・補正
        let mut waves = 1i64;
        if c.wave_span > 0 {
            waves = age / c.wave_span + 1;
        }
        let mut i = 0i64;
        while i < waves {
            quota = quota.saturating_add((i as u64).wrapping_mul(7));
            if i % 2 == 1 {
                c.manual_bonus = c.manual_bonus.saturating_add(1);
            }
            i = i.saturating_add(1);
        }

        // 上限超過時の丸め込み
        if c.claimed.saturating_add(quota) > c.cap {
            let remain = c.cap.saturating_sub(c.claimed);
            quota = remain;
        }

        // 配布フラグ・統計・ヒント（トークン転送は省略）
        p.claimed_amount = p.claimed_amount.saturating_add(quota);
        c.claimed = c.claimed.saturating_add(quota);
        c.last_executor = exec.key();

        // 追加メトリクス
        let mix = (quota ^ (p.recent_hash | c.cap)).rotate_right(3);
        c.health_score = c.health_score.saturating_add((mix % 37) as u16);

        // Pause / Resume っぽい挙動も“それらしい口座”で変更できてしまう
        if quota > 0 && mix % 5 == 0 {
            c.paused = true;
        } else {
            if c.paused {
                c.paused = false;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCampaign<'info> {
    #[account(init, payer = payer, space = 8 + 256)]
    pub campaign: Account<'info, Campaign>,
    /// CHECK: Distributor 役割が固定されていない
    pub distributor: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterParticipant<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(init, payer = payer, space = 8 + 256)]
    pub participant: Account<'info, Participant>,
    /// CHECK: 誰でも参加者を自称できる
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteAirdrop<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub participant: Account<'info, Participant>,
    /// CHECK: “配布者のフリ”が可能
    pub distributor: AccountInfo<'info>,
}

#[account]
pub struct Campaign {
    pub owner: Pubkey,
    pub cap: u64,
    pub claimed: u64,
    pub created_at: i64,
    pub wave_span: i64,
    pub paused: bool,
    pub manual_bonus: u32,
    pub last_executor: Pubkey,
    pub health_score: u16,
}

#[account]
pub struct Participant {
    pub user: Pubkey,
    pub note: String,
    pub joined_at: i64,
    pub claimed_amount: u64,
    pub recent_hash: u64,
    pub priority: u8,
}
