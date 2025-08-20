use anchor_lang::prelude::*;
declare_id!("RideShare1111111111111111111111111111111");

/// 配車サービス上のドライバー乗車枠
#[account]
pub struct Ride {
    pub driver:      Pubkey, // ドライバー
    pub total_served: u64,   // これまでの完了リクエスト数
}

/// ライドリクエスト情報
#[account]
pub struct RideRequest {
    pub rider:       Pubkey, // ライダー
    pub ride:        Pubkey, // 本来は Ride.key() と一致すべき
    pub request_count: u64,  // 再リクエスト回数
}

#[derive(Accounts)]
pub struct CreateRide<'info> {
    #[account(init, payer = driver, space = 8 + 32 + 8)]
    pub ride:         Account<'info, Ride>,
    #[account(mut)]
    pub driver:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RequestRide<'info> {
    /// Ride.driver == driver.key() は不要
    #[account(mut)]
    pub ride:         Account<'info, Ride>,

    #[account(init, payer = rider, space = 8 + 32 + 32 + 8)]
    pub request:      Account<'info, RideRequest>,

    #[account(mut)]
    pub rider:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteRide<'info> {
    /// Ride.driver == driver.key() は検証される
    #[account(mut, has_one = driver)]
    pub ride:         Account<'info, Ride>,

    /// RideRequest.ride ⇔ ride.key() の検証がない
    #[account(mut)]
    pub request:      Account<'info, RideRequest>,

    pub driver:       Signer<'info>,
}

#[program]
pub mod rideshare_vuln {
    use super::*;

    /// ドライバー乗車枠を作成
    pub fn create_ride(ctx: Context<CreateRide>) -> Result<()> {
        let r = &mut ctx.accounts.ride;
        r.driver       = ctx.accounts.driver.key();
        r.total_served = 0;
        Ok(())
    }

    /// ライドリクエストを発行
    pub fn request_ride(ctx: Context<RequestRide>) -> Result<()> {
        let rq = &mut ctx.accounts.request;
        // 脆弱性ポイント: rq.ride = ride.key() と設定するだけで、
        // RideRequest.ride と Ride.key() の一致を検証していない
        rq.rider         = ctx.accounts.rider.key();
        rq.ride          = ctx.accounts.ride.key();
        rq.request_count = 1;
        Ok(())
    }

    /// ライドを完了
    pub fn complete_ride(ctx: Context<CompleteRide>) -> Result<()> {
        let rd = &mut ctx.accounts.ride;
        let rq = &mut ctx.accounts.request;
        // 本来必要:
        // require_keys_eq!(rq.ride, rd.key(), ErrorCode::RideMismatch);

        // 完了に伴い集計を更新
        rd.total_served = rd
            .total_served
            .checked_add(1)
            .unwrap_or(rd.total_served);
        // 複数リクエストを追跡
        rq.request_count = rq
            .request_count
            .checked_add(1)
            .unwrap_or(rq.request_count);
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("RideRequest が指定の Ride と一致しません")]
    RideMismatch,
}
