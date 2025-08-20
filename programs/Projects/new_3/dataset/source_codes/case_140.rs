use anchor_lang::prelude::*;
declare_id!("RefeRralPro1111111111111111111111111111111");

/// 紹介報酬プログラム情報
#[account]
pub struct ReferralProgram {
    pub program_owner: Pubkey, // プログラム管理者
    pub total_rewards: u64,    // 残高報酬額
}

/// ユーザー紹介記録
#[account]
pub struct ReferralRecord {
    pub referred_count: u64,   // これまでの紹介数
    pub program_id:     Pubkey,// 本来は ReferralProgram.key() と一致すべき
}

/// 紹介報酬支払いイベント
#[event]
pub struct ReferralAwarded {
    pub program: Pubkey,
    pub record:  Pubkey,
    pub amount:  u64,
}

#[derive(Accounts)]
pub struct AwardReferral<'info> {
    /// ReferralProgram.program_owner == program_owner.key() はチェックされる
    #[account(mut, has_one = program_owner)]
    pub program:      Account<'info, ReferralProgram>,

    /// しかし ReferralRecord.program_id == program.key() の検証は一切ない
    #[account(mut)]
    pub record:       Account<'info, ReferralRecord>,

    pub program_owner: Signer<'info>,
}

#[program]
pub mod referral_vuln {
    use super::*;

    pub fn award(ctx: Context<AwardReferral>, amount: u64) -> Result<()> {
        let p = &mut ctx.accounts.program;
        let r = &mut ctx.accounts.record;

        // 紐付けチェックがないため、攻撃者は自分で用意した
        // ReferralRecord アカウントを渡して好きに報酬を受け取れる

        p.total_rewards = p.total_rewards.checked_sub(amount).unwrap();
        r.referred_count = r.referred_count.checked_add(amount).unwrap();
        r.program_id     = p.key();

        emit!(ReferralAwarded {
            program: p.key(),
            record:  r.key(),
            amount,
        });

        Ok(())
    }
}
