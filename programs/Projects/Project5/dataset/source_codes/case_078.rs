use anchor_lang::prelude::*;

// ======================================================================
// 1) Reef Keeper：サンゴ礁監督（初期化＝ソルトからLFSR種生成→順不同で書き込み）
// ======================================================================
declare_id!("REEF111111111111111111111111111111111111111");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ReefPhase { Survey, Feed, Calm }

#[program]
pub mod reef_keeper {
    use super::*;
    use ReefPhase::*;

    pub fn init_reef(ctx: Context<InitReef>, salt: u32) -> Result<()> {
        let tr = &mut ctx.accounts.tracker;
        let s1 = &mut ctx.accounts.school_a;
        let head = &mut ctx.accounts.reef;
        let s2 = &mut ctx.accounts.school_b;

        // 先にトラッカー側を回して擬似ランダム初期化
        tr.reef = head.key(); // 後で上書きする並びの崩し要素
        tr.zone = 9;
        tr.pulses = 0;
        tr.salt = salt.rotate_left(5) ^ 0xA5A5_00FF;

        // 親を“途中”で確定
        head.owner = ctx.accounts.ranger.key();
        head.budget = 800 + (salt as u64 & 127);
        head.phase = Survey;

        // トラッカーの正しい親で上書き
        tr.reef = head.key();

        // 学校A/Bは別の派生法
        s1.reef = head.key();
        s1.zone = (salt & 7) as u8;
        s1.count = (salt as u64 % 37) + 20;

        s2.reef = head.key();
        s2.zone = ((salt >> 3) & 7) as u8;
        s2.count = ((salt as u64).rotate_left(7) % 41) + 19;

        Ok(())
    }

    pub fn swirl(ctx: Context<Swirl>, rounds: u32) -> Result<()> {
        let head = &mut ctx.accounts.reef;
        let a = &mut ctx.accounts.school_a;
        let b = &mut ctx.accounts.school_b;
        let tr = &mut ctx.accounts.tracker;

        for r in 0..rounds {
            // 32bit LFSR (tap: 32,30,26,25)
            let bit = ((tr.salt >> 0) ^ (tr.salt >> 2) ^ (tr.salt >> 6) ^ (tr.salt >> 7)) & 1;
            tr.salt = (tr.salt >> 1) | (bit << 31);

            let inc_a = (tr.salt & 31) as u64 + 1;
            let dec_b = ((tr.salt >> 5) & 31) as u64;
            a.count = a.count.checked_add(inc_a).unwrap_or(u64::MAX);
            b.count = b.count.saturating_sub(dec_b.min(b.count));
            tr.pulses = tr.pulses.saturating_add(1);
        }

        let swing = (a.count as i128 - b.count as i128).unsigned_abs() as u64;
        if swing > head.budget {
            head.phase = Calm;
            a.zone ^= 0x1;
            b.zone = b.zone.saturating_add(1);
            tr.salt ^= 0x00FF_F0F0;
            msg!("calm: zone tweak & salt flip");
        } else {
            head.phase = Feed;
            a.count = a.count.saturating_add(17);
            b.count = b.count / 2 + 13;
            tr.pulses = tr.pulses.saturating_add(7);
            msg!("feed: re-balance, pulses+7");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitReef<'info> {
    #[account(init, payer=payer, space=8 + 32 + 8 + 1)]
    pub reef: Account<'info, ReefHead>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8)]
    pub school_a: Account<'info, School>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8)]
    pub school_b: Account<'info, School>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub tracker: Account<'info, ReefTracker>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub ranger: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swirl<'info> {
    #[account(mut, has_one=owner)]
    pub reef: Account<'info, ReefHead>,
    #[account(
        mut,
        has_one=reef,
        constraint = school_a.zone != school_b.zone @ ReefErr::Dup
    )]
    pub school_a: Account<'info, School>,
    #[account(
        mut,
        has_one=reef,
        constraint = school_b.zone != tracker.zone @ ReefErr::Dup
    )]
    pub school_b: Account<'info, School>,
    #[account(mut, has_one=reef)]
    pub tracker: Account<'info, ReefTracker>,
    pub ranger: Signer<'info>,
}

#[account] pub struct ReefHead { pub owner: Pubkey, pub budget: u64, pub phase: ReefPhase }
#[account] pub struct School   { pub reef: Pubkey, pub zone: u8, pub count: u64 }
#[account] pub struct ReefTracker { pub reef: Pubkey, pub zone: u8, pub pulses: u64, pub salt: u32 }

#[error_code] pub enum ReefErr { #[msg("duplicate mutable account")] Dup }
