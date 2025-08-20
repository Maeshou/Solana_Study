use anchor_lang::prelude::*;
declare_id!("RentNFT22222222222222222222222222222222222");

/// レンタル契約情報
#[account]
pub struct RentContract {
    pub owner:         Pubkey, // レンタル提供者
    pub total_rentals: u64,    // 総レンタル数
    pub fee:           u64,    // レンタル料金（lamports）
}

/// NFT 資産情報
#[account]
pub struct NFTAsset {
    pub mint:            Pubkey, // NFT の Mint アドレス
    pub owner:           Pubkey, // 現在の所有者
    pub rental_contract: Pubkey, // 本来は RentContract.key() と一致すべき
    pub active:          bool,   // レンタル中フラグ
}

#[derive(Accounts)]
pub struct InitializeContract<'info> {
    #[account(init, payer = provider, space = 8 + 32 + 8 + 8)]
    pub rent_contract: Account<'info, RentContract>,
    #[account(mut)]
    pub provider:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RentAsset<'info> {
    /// RentContract.owner == renter.key() は検証される
    #[account(mut, has_one = owner)]
    pub rent_contract: Account<'info, RentContract>,

    /// NFTAsset.rental_contract と rent_contract.key() の検証がない
    #[account(mut)]
    pub nft_asset:     Account<'info, NFTAsset>,

    /// レンタル申請者
    pub owner:         Signer<'info>,
}

#[derive(Accounts)]
pub struct EndRental<'info> {
    /// RentContract.owner == provider.key() は検証される
    #[account(mut, has_one = owner)]
    pub rent_contract: Account<'info, RentContract>,

    /// 同様に一致検証なし
    #[account(mut)]
    pub nft_asset:     Account<'info, NFTAsset>,

    /// 契約者
    pub owner:         Signer<'info>,
}

#[program]
pub mod nft_rental_vuln2 {
    use super::*;

    /// レンタル契約を初期化
    pub fn initialize_contract(
        ctx: Context<InitializeContract>,
        fee: u64
    ) -> Result<()> {
        let rc = &mut ctx.accounts.rent_contract;
        rc.owner         = ctx.accounts.provider.key();
        rc.total_rentals = 0;
        rc.fee           = fee;
        Ok(())
    }

    /// NFT をレンタル
    pub fn rent_asset(ctx: Context<RentAsset>) -> Result<()> {
        let rc = &mut ctx.accounts.rent_contract;
        let na = &mut ctx.accounts.nft_asset;

        // 脆弱性ポイント：
        // na.rental_contract = rc.key(); と設定するだけで、
        // NFTAsset.rental_contract と RentContract.key() の一致検証が行われない
        na.owner           = ctx.accounts.owner.key();
        na.rental_contract = rc.key();
        na.active          = true;

        rc.total_rentals = rc.total_rentals.checked_add(1).unwrap();
        Ok(())
    }

    /// レンタル終了（返却）
    pub fn end_rental(ctx: Context<EndRental>) -> Result<()> {
        let rc = &mut ctx.accounts.rent_contract;
        let na = &mut ctx.accounts.nft_asset;

        // 本来は必須：
        // require_keys_eq!(
        //     na.rental_contract,
        //     rc.key(),
        //     ErrorCode::ContractMismatch
        // );
        na.owner  = rc.owner;   // 元の所有者に返却
        na.active = false;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("NFTAsset が指定の RentContract と一致しません")]
    ContractMismatch,
}
