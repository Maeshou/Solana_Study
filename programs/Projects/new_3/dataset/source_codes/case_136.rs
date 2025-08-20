use anchor_lang::prelude::*;

declare_id!("RentNFT1111111111111111111111111111111111111");

/// レンタル契約情報
#[account]
pub struct RentContract {
    pub owner:          Pubkey, // レンタル提供者
    pub nft_contract:   Pubkey, // レンタル対象のコントラクトアドレス（Mint）
    pub rent_fee:       u64,    // レンタル料金（lamports）
    pub renter:         Pubkey, // レンタル利用者
    pub expires_at:     i64,    // レンタル終了時刻（UNIXタイム）
}

/// NFT資産情報（外部から渡されるAccount）
#[account]
pub struct NFTAsset {
    pub mint:            Pubkey, // この NFT の Mint アドレス
    pub owner:           Pubkey, // 現在の所有者
    pub rental_contract: Pubkey, // 本来は RentContract.key() と一致すべき
    pub is_rented:       bool,   // レンタル中フラグ
}

/// レンタル開始イベント
#[event]
pub struct NftRented {
    pub contract: Pubkey,
    pub asset:    Pubkey,
    pub renter:   Pubkey,
}

/// レンタル開始
#[derive(Accounts)]
pub struct InitializeContract<'info> {
    #[account(init, payer = owner, space = 8 + 32*4 + 8 + 1)]
    pub rent_contract: Account<'info, RentContract>,
    #[account(mut)]
    pub owner:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// NFTを借りる
#[derive(Accounts)]
pub struct RentNft<'info> {
    /// RentContract.renter == renter.key() は検証される
    #[account(mut, has_one = renter)]
    pub rent_contract: Account<'info, RentContract>,

    /// 本来は rent_contract.key() と NFTAsset.rental_contract が一致すべきだが、
    /// ここでは一切チェックしていない
    #[account(mut)]
    pub nft_asset:     Account<'info, NFTAsset>,

    pub renter:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// NFTを返却する
#[derive(Accounts)]
pub struct ReturnNft<'info> {
    /// レンタル契約の所有者だけが返却を承認できる
    #[account(mut, has_one = owner)]
    pub rent_contract: Account<'info, RentContract>,

    /// 同上。紐づき検証なし
    #[account(mut)]
    pub nft_asset:     Account<'info, NFTAsset>,

    pub owner:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod nft_rental_vuln {
    use super::*;

    /// レンタル契約を初期化
    pub fn initialize_contract(ctx: Context<InitializeContract>, nft_contract: Pubkey, fee: u64, duration_secs: i64) -> Result<()> {
        let c = &mut ctx.accounts.rent_contract;
        c.owner        = ctx.accounts.owner.key();
        c.nft_contract = nft_contract;
        c.rent_fee     = fee;
        c.renter       = Pubkey::default();
        c.expires_at   = 0;
        Ok(())
    }

    /// NFTを借りる
    pub fn rent(ctx: Context<RentNft>) -> Result<()> {
        let c    = &mut ctx.accounts.rent_contract;
        let asset = &mut ctx.accounts.nft_asset;

        // 本来は必須：
        // require_keys_eq!(
        //     asset.rental_contract,
        //     c.key(),
        //     RentalError::ContractMismatch
        // );
        // もしくは
        // #[account(address = rent_contract.key())]
        // pub nft_asset: Account<'info, NFTAsset>,

        // チェックがないため攻撃者は任意のNFTAssetを渡せる
        asset.owner           = ctx.accounts.renter.key();
        asset.is_rented       = true;
        asset.rental_contract = c.key();
        c.renter              = ctx.accounts.renter.key();
        c.expires_at          = Clock::get()?.unix_timestamp + 60 * 60 * 24; // 24時間レンタル

        emit!(NftRented {
            contract: c.key(),
            asset:    asset.key(),
            renter:   c.renter,
        });
        msg!("NFT {} をレンタルしました (契約: {})", asset.mint, c.key());
        Ok(())
    }

    /// NFTを返却する
    pub fn return_nft(ctx: Context<ReturnNft>) -> Result<()> {
        let c     = &mut ctx.accounts.rent_contract;
        let asset = &mut ctx.accounts.nft_asset;

        // ここでも asset.rental_contract == c.key() のチェックがないため、
        // 異なる契約のNFTを返却させることが可能になってしまう。

        asset.owner           = c.owner;
        asset.is_rented       = false;
        asset.rental_contract = Pubkey::default();
        c.renter              = Pubkey::default();
        c.expires_at          = 0;

        msg!("NFT {} を返却しました (契約: {})", asset.mint, c.key());
        Ok(())
    }
}

#[error_code]
pub enum RentalError {
    #[msg("NFTAsset が指定されたレンタル契約と一致しません")]
    ContractMismatch,
}
