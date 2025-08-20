use anchor_lang::prelude::*;

declare_id!("R8eNtLtEnD4CcT1oNbAbCdEfGhIjKlMnOpQrStUv");

#[program]
pub mod nft_rental_service {
    use super::*;

    /// owner_account から renter_account に NFT をレンタルするが、
    /// 同一アカウントチェックが入っていないため、
    /// owner_account.key() == renter_account.key() でも “自己レンタル” が成立してしまう
    pub fn rent_nft(
        ctx: Context<RentNft>,
        duration_secs: i64,
        memo: String,
    ) -> ProgramResult {
        let owner_acc  = &mut ctx.accounts.owner_account;
        let renter_acc = &mut ctx.accounts.renter_account;
        let nft         = &mut ctx.accounts.nft;
        let now         = ctx.accounts.clock.unix_timestamp;

        // DuplicateMutableAccount リスクを無視して処理が続行される

        // レンタル回数をインクリメント
        owner_acc.rental_count  = owner_acc.rental_count + 1;
        renter_acc.rental_count = renter_acc.rental_count + 1;

        // NFT ステータスを更新
        nft.is_rented     = true;
        nft.rental_start  = now;
        nft.rental_end    = now + duration_secs;

        // 最終アクション時刻を同期
        owner_acc.last_action  = now;
        renter_acc.last_action = now;

        // メモ欄に履歴を追加
        owner_acc.notes  = owner_acc.notes.clone()  + "|rent@"  + &now.to_string() + ":" + &memo;
        renter_acc.notes = renter_acc.notes.clone() + "|lend@"  + &now.to_string() + ":" + &memo;

        msg!(
            "Rental started: NFT {} rented by {} from {} for {} seconds",
            nft.id,
            ctx.accounts.renter.key(),
            ctx.accounts.owner.key(),
            duration_secs
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RentNft<'info> {
    /// NFT の貸し手プロファイル（mutable）
    #[account(mut)]
    pub owner_account:  Account<'info, UserProfile>,

    /// NFT の借り手プロファイル（mutable）
    #[account(mut)]
    pub renter_account: Account<'info, UserProfile>,

    /// レンタル対象 NFT（mutable）
    #[account(mut)]
    pub nft:             Account<'info, GameNft>,

    /// 操作を実行する署名者
    #[account(signer)]
    pub operator:        Signer<'info>,

    /// 時刻取得用 Sysvar
    pub clock:           Sysvar<'info, Clock>,

    /// システムプログラム
    pub system_program:  Program<'info, System>,
}

#[account]
pub struct UserProfile {
    /// このプロファイルのオーナー
    pub owner:          Pubkey,
    /// 累積レンタル回数
    pub rental_count:   u32,
    /// 最終操作 UNIX タイムスタンプ
    pub last_action:    i64,
    /// 任意メモ欄
    pub notes:          String,
}

#[account]
pub struct GameNft {
    /// NFT の一意 ID
    pub id:             u64,
    /// 現在のオーナー
    pub owner:          Pubkey,
    /// レンタル中かどうか
    pub is_rented:      bool,
    /// レンタル開始時刻
    pub rental_start:   i64,
    /// レンタル終了予定時刻
    pub rental_end:     i64,
}
