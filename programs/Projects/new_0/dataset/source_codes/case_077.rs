use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfVEST01");

#[program]
pub mod time_locked_vesting {
    use super::*;

    /// 初回のみベスティングアカウントを初期化し、
    /// 受取人、総額、解放タイムスタンプを設定します。
    pub fn initialize_vesting(
        ctx: Context<InitializeVesting>,
        amount: u64,
        release_time: i64,
    ) -> Result<()> {
        let vest = &mut ctx.accounts.vesting;
        vest.beneficiary = ctx.accounts.beneficiary.key();
        vest.total_amount = amount;
        vest.release_time = release_time;
        vest.claimed_amount = 0;
        Ok(())
    }

    /// 解放時刻到来後に一度だけ全額を請求します（分岐・ループなし）。
    pub fn claim_vested(ctx: Context<ClaimVested>) -> Result<()> {
        let vest = &mut ctx.accounts.vesting;
        // 現在のUnix時刻
        let now = Clock::get()?.unix_timestamp;
        // 解放までの猶予をsaturating_subで計算（時刻未到来なら0）
        let elapsed = now.saturating_sub(vest.release_time) as u64;
        // elapsed > 0なら全額請求、0なら請求ゼロ
        let can = (elapsed != 0) as u64;
        let payable = vest.total_amount.saturating_mul(can);
        // 既に請求済み量との差分（初回なら全額）
        let to_claim = payable.saturating_sub(vest.claimed_amount);
        vest.claimed_amount = vest.claimed_amount.saturating_add(to_claim);
        msg!(
            "Vested {} tokens to {:?}",
            to_claim,
            vest.beneficiary
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVesting<'info> {
    /// 初回のみPDAを作成・初期化
    #[account(
        init,
        payer = authority,
        space  = 8 + 32 + 8 + 8 + 8,
        seeds = [b"vesting", beneficiary.key().as_ref()],
        bump
    )]
    pub vesting: Account<'info, Vesting>,

    /// 初期化トランザクションの署名者
    #[account(mut)]
    pub authority: Signer<'info>,

    /// ベスティングを受け取るアドレス
    /// (Signer でもよいが、view-only の場合は指定外可能)
    pub beneficiary: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimVested<'info> {
    /// PDA と所有者チェックで不正アクセス防止
    #[account(
        seeds = [b"vesting", beneficiary.key().as_ref()],
        bump,
        has_one = beneficiary
    )]
    pub vesting: Account<'info, Vesting>,

    /// 請求者は必ずベネフィシャリ
    pub beneficiary: Signer<'info>,
}

#[account]
pub struct Vesting {
    /// 受取人
    pub beneficiary: Pubkey,
    /// 合計ベスティング額
    pub total_amount: u64,
    /// UNIX秒での解放時刻
    pub release_time: i64,
    /// 既に請求した量
    pub claimed_amount: u64,
}
