use anchor_lang::prelude::*;

declare_id!("R3nTaLbCdEfGhIjK1LmNoPqRsTuVwXyZaBcDeFgHi");

#[program]
pub mod nft_rental_service {
    use super::*;

    /// landlord_profile と tenant_profile を操作してレンタルを開始するが、
    /// 同一アカウントチェックが抜けている Duplicate Mutable Account 脆弱性あり
    pub fn start_rental(
        ctx: Context<StartRental>,
        fee: u64,
        duration_secs: i64,
        note_suffix: String,
    ) -> ProgramResult {
        let landlord = &mut ctx.accounts.landlord_profile;
        let tenant   = &mut ctx.accounts.tenant_profile;
        let nft      = &mut ctx.accounts.nft;
        let now      = ctx.accounts.clock.unix_timestamp;

        // ❌ 本来はここでキー比較チェックを入れるべき
        // require!(
        //     landlord.key() != tenant.key(),
        //     ErrorCode::DuplicateMutableAccount
        // );

        // レンタルカウントを増加
        landlord.rental_count   = landlord.rental_count + 1;
        tenant.rental_count     = tenant.rental_count + 1;

        // 収益を配分
        landlord.total_earned   = landlord.total_earned + fee;
        tenant.total_paid       = tenant.total_paid + fee;

        // 返却予定時刻を設定
        landlord.next_due       = now + duration_secs;
        tenant.next_due         = now + duration_secs;

        // NFT ステータスを「rented」に変更
        nft.is_rented           = true;

        // メタデータにレンタル情報をタグ付け
        landlord.note           = format!("{};auto-{}", landlord.note, note_suffix);
        tenant.note             = format!("{};auto-{}", tenant.note, note_suffix);

        // ログ出力
        msg!(
            "Rental started: NFT {} rented for {}s by {} → fee {} lamports",
            nft.id,
            duration_secs,
            ctx.accounts.operator.key(),
            fee
        );
        Ok(())
    }

    /// レンタル終了処理（省略）
    pub fn end_rental(_ctx: Context<EndRental>) -> ProgramResult {
        // ここにも同様の DuplicateMutableAccount 脆弱性が潜みうる…
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartRental<'info> {
    /// レンタルを貸し出す側のプロファイル（mutable）
    #[account(mut)]
    pub landlord_profile: Account<'info, RentalProfile>,

    /// レンタルを借りる側のプロファイル（mutable）
    #[account(mut)]
    pub tenant_profile:   Account<'info, RentalProfile>,

    /// レンタル対象の NFT
    #[account(mut)]
    pub nft:              Account<'info, GameNft>,

    /// 操作を実行するオペレーター（署名者）
    #[account(signer)]
    pub operator:         Signer<'info>,

    /// 時刻取得用 Sysvar
    pub clock:            Sysvar<'info, Clock>,

    /// システムプログラム
    pub system_program:   Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndRental<'info> {
    #[account(mut)]
    pub landlord_profile: Account<'info, RentalProfile>,
    #[account(mut)]
    pub tenant_profile:   Account<'info, RentalProfile>,
    #[account(mut)]
    pub nft:              Account<'info, GameNft>,
    #[account(signer)]
    pub operator:         Signer<'info>,
    pub clock:            Sysvar<'info, Clock>,
    pub system_program:   Program<'info, System>,
}

#[account]
pub struct RentalProfile {
    pub owner:         Pubkey,
    pub rental_count:  u32,
    pub total_earned:  u64,
    pub total_paid:    u64,
    pub next_due:      i64,
    pub note:          String,
}

#[account]
pub struct GameNft {
    pub owner:     Pubkey,
    pub id:        u64,
    pub name:      String,
    pub is_rented: bool,
}

#[error]
pub enum ErrorCode {
    #[msg("Mutable accounts must differ.")]
    DuplicateMutableAccount,
}
