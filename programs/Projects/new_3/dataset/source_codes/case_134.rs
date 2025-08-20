use anchor_lang::prelude::*;

#[account]
pub struct VestingSchedule {
    pub beneficiary: Pubkey, // この beneficiary と署名者が一致するかは has_one でチェック
    pub total_amount: u64,
}

#[account]
pub struct ClaimRecord {
    pub claimed: u64,
    pub schedule: Pubkey, // 本来はここが VestingSchedule.key() と一致すべき
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    /// VestingSchedule.beneficiary == beneficiary.key() は検証される
    #[account(mut, has_one = beneficiary)]
    pub vesting_schedule: Account<'info, VestingSchedule>,

    /// ClaimRecord.schedule == vesting_schedule.key() の検証がないため、
    /// 別のスケジュール用に attacker が用意したレコードでも通ってしまう
    #[account(mut)]
    pub claim_record: Account<'info, ClaimRecord>,

    pub beneficiary: Signer<'info>,
}

#[program]
pub mod vesting_vuln {
    use super::*;

    pub fn claim(ctx: Context<ClaimTokens>, amount: u64) -> Result<()> {
        // 本来は以下のどちらかが必要：
        // 1) 手動チェック
        //    require_keys_eq!(
        //        ctx.accounts.claim_record.schedule,
        //        ctx.accounts.vesting_schedule.key(),
        //        VestingError::RecordMismatch
        //    );
        // 2) #[account(address = vesting_schedule.key())]
        //    pub claim_record: Account<'info, ClaimRecord>,

        // どちらもないため、攻撃者は自分用の ClaimRecord を用意し、
        // schedule フィールドに beneficiary 承認済みのスケジュールIDをセットして渡せば、
        // 任意の額を何度でもクレームできてしまう。
        ctx.accounts.claim_record.claimed = ctx
            .accounts
            .claim_record
            .claimed
            .checked_add(amount)
            .unwrap();

        // さらに実際のトークン転送処理などに進む……
        Ok(())
    }
}

#[error_code]
pub enum VestingError {
    #[msg("ClaimRecord が指定された VestingSchedule と一致しません")]
    RecordMismatch,
}
