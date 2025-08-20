use anchor_lang::prelude::*;
declare_id!("NftRentSafe1111111111111111111111111111111");

/// レントコントラクト情報
#[account]
pub struct RentContract {
    pub owner:      Pubkey,  // コントラクト作成者
    pub fee_per_day:u64,     // レンタル料金（lamports/日）
}

/// NFT 資産情報
#[account]
pub struct NFTAsset {
    pub owner:        Pubkey,  // 元オーナー
    pub contract:     Pubkey,  // RentContract.key()
    pub rented_to:    Option<Pubkey>, // 現在の借り手（None = 未貸出）
}

/// レンタル履歴
#[account]
pub struct RentalRecord {
    pub asset:        Pubkey,  // NFTAsset.key()
    pub renter:       Pubkey,  // 借り手
    pub start_ts:     i64,     // 開始時刻
}

#[derive(Accounts)]
pub struct InitializeContract<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 8)]
    pub contract:    Account<'info, RentContract>,
    #[account(mut)]
    pub creator:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListAsset<'info> {
    /// RentContract.owner == creator.key() を検証
    #[account(mut, has_one = owner)]
    pub contract:    Account<'info, RentContract>,

    /// NFTAsset.contract == contract.key()、NFTAsset.owner == owner.key() を検証
    #[account(init, payer = owner, space = 8 + 32 + 32 + 1,
        has_one = contract,
        has_one = owner)]
    pub asset:       Account<'info, NFTAsset>,

    #[account(mut)]
    pub owner:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RentNFT<'info> {
    /// NFTAsset.contract == contract.key() を検証
    #[account(mut, has_one = contract)]
    pub asset:       Account<'info, NFTAsset>,

    /// RentContract.key() の照合は上で担保
    #[account(mut)]
    pub contract:    Account<'info, RentContract>,

    /// RentalRecord.asset == asset.key()、renter == renter.key() を検証
    #[account(init, payer = renter, space = 8 + 32 + 32 + 8,
        has_one = asset,
        has_one = renter)]
    pub record:      Account<'info, RentalRecord>,

    #[account(mut)]
    pub renter:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReturnNFT<'info> {
    /// RentalRecord.asset == asset.key()、renter == caller.key() を検証
    #[account(mut, has_one = asset, has_one = renter)]
    pub record:      Account<'info, RentalRecord>,

    /// NFTAsset.contract == contract.key() を検証
    #[account(mut, has_one = contract)]
    pub asset:       Account<'info, NFTAsset>,

    #[account(mut)]
    pub contract:    Account<'info, RentContract>,

    pub renter:      Signer<'info>,
}

#[program]
pub mod nft_rental_safe {
    use super::*;

    pub fn initialize_contract(
        ctx: Context<InitializeContract>,
        fee_per_day: u64
    ) -> Result<()> {
        let c = &mut ctx.accounts.contract;
        c.owner       = ctx.accounts.creator.key();
        c.fee_per_day = fee_per_day;
        Ok(())
    }

    pub fn list_asset(ctx: Context<ListAsset>) -> Result<()> {
        let a = &mut ctx.accounts.asset;
        // Anchor が has_one 属性で owner & contract の一致をチェック
        a.owner     = ctx.accounts.owner.key();
        a.contract  = ctx.accounts.contract.key();
        a.rented_to = None;
        Ok(())
    }

    pub fn rent_nft(ctx: Context<RentNFT>) -> Result<()> {
        let a = &mut ctx.accounts.asset;
        let r = &mut ctx.accounts.record;

        // 二重チェック（optional）
        require_keys_eq!(r.asset, a.key(), NftError::AssetMismatch);
        require_keys_eq!(r.renter, ctx.accounts.renter.key(), NftError::RenterMismatch);

        // 貸し出し中でないことを確認
        require!(a.rented_to.is_none(), NftError::AlreadyRented);

        // レンタル開始
        a.rented_to = Some(ctx.accounts.renter.key());
        r.asset     = a.key();
        r.renter    = ctx.accounts.renter.key();
        r.start_ts  = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn return_nft(ctx: Context<ReturnNFT>) -> Result<()> {
        let a = &mut ctx.accounts.asset;

        // 二重チェック
        require_keys_eq!(ctx.accounts.record.asset, a.key(), NftError::AssetMismatch);
        require_keys_eq!(ctx.accounts.record.renter, ctx.accounts.renter.key(), NftError::RenterMismatch);

        // 貸し出し中であることを確認
        require!(a.rented_to.is_some(), NftError::NotRented);

        // 返却処理
        a.rented_to = None;
        Ok(())
    }
}

#[error_code]
pub enum NftError {
    #[msg("Asset と RentalRecord.asset が一致しません")]
    AssetMismatch,
    #[msg("Renter が一致しません")]
    RenterMismatch,
    #[msg("すでに貸し出されています")]
    AlreadyRented,
    #[msg("現在貸し出されていません")]
    NotRented,
}
