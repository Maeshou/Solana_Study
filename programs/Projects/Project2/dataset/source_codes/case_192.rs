use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct ParkingLot(pub u8, pub Vec<(Pubkey, i64)>); // (bump, Vec<(vehicle, entry_ts)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVD");

#[error_code]
pub enum ErrorCode {
    #[msg("Parking lot is full")]
    LotFull,
    #[msg("Vehicle not found")]
    VehicleNotFound,
}

#[program]
pub mod parking_lot {
    use super::*;

    const MAX_VEHICLES: usize = 50;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("lot").unwrap();
        ctx.accounts.lot.0 = b;
        Ok(())
    }

    /// 乗り入れ：台数制限チェック＋現在時刻で追加
    pub fn enter(ctx: Context<Modify>, vehicle: Pubkey) -> Result<()> {
        let list = &mut ctx.accounts.lot.1;
        if list.len() >= MAX_VEHICLES {
            return err!(ErrorCode::LotFull);
        }
        let now = ctx.accounts.clock.unix_timestamp;
        list.push((vehicle, now));
        Ok(())
    }

    /// 退場：該当車両を一括削除（入場時刻は不要に）
    pub fn exit(ctx: Context<Modify>, vehicle: Pubkey) -> Result<()> {
        let list = &mut ctx.accounts.lot.1;
        list.retain(|&(v, _)| {
            if v == vehicle {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 現在台数報告：ログ出力
    pub fn count_cars(ctx: Context<Modify>) -> Result<()> {
        let cnt = ctx.accounts.lot.1.len() as u64;
        msg!("Current parked vehicles: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"lot", authority.key().as_ref()],
        bump,
        // discriminator(8)+bump(1)+Vec len(4)+max50*(32+8)
        space = 8 + 1 + 4 + 50 * (32 + 8)
    )]
    pub lot:       Account<'info, ParkingLot>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"lot", authority.key().as_ref()],
        bump = lot.0,
    )]
    pub lot:       Account<'info, ParkingLot>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
