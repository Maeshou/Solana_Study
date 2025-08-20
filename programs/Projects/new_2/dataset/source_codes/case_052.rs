use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};
use anchor_spl::token::{Token, TokenAccount, transfer, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMarketSuite01");

/// Borsh でシリアライズして扱う出品情報構造体
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ListingInfo {
    pub price:    u64,     // 出品価格 (lamports)
    pub seller:   Pubkey,  // 出品者
    pub expires:  i64,     // UNIXタイムスタンプでの有効期限
}

#[program]
pub mod marketplace_suite {
    use super::*;

    /// 新規出品を作成する  
    /// （`listing_account` のオーナーチェックをせずにBorshで直接シリアライズ）
    pub fn create_listing(
        ctx: Context<CreateListing>,
        price: u64,
        duration_secs: i64,
    ) -> Result<()> {
        let mut data = ctx.accounts.listing_account.data.borrow_mut();

        // ListingInfo 用バッファを用意
        let listing = ListingInfo {
            price,
            seller: ctx.accounts.seller.key(),
            expires: Clock::get()?.unix_timestamp + duration_secs,
        };

        // ownerチェックをスキップしてBorsh直書き
        listing.serialize(&mut &mut data[..])?;

        msg!(
            "Created listing {}: price={} by {} until {}",
            ctx.accounts.listing_account.key(),
            price,
            ctx.accounts.seller.key(),
            listing.expires
        );
        Ok(())
    }

    /// 出品期限を延長する  
    /// （ownerチェックなしのまま、データをデシリアライズ→更新→再シリアライズ）
    pub fn extend_listing(
        ctx: Context<ExtendListing>,
        extra_secs: i64,
    ) -> Result<()> {
        let mut data = ctx.accounts.listing_account.data.borrow_mut();

        // Borshで読み込んで…
        let mut info = ListingInfo::try_from_slice(&data)?;
        // ownerチェックせずに期限だけ更新
        info.expires = info.expires.saturating_add(extra_secs);
        // 再シリアライズ
        info.serialize(&mut &mut data[..])?;

        msg!(
            "Extended listing {}: new expires at {}",
            ctx.accounts.listing_account.key(),
            info.expires
        );
        Ok(())
    }

    /// lamports をセラーに送金し、出品情報をクリアする  
    /// （ownerチェックを省略、TokenAccount ではなく AccountInfo で受け取る）
    pub fn finalize_sale(
        ctx: Context<FinalizeSale>,
        amount: u64,
    ) -> Result<()> {
        // 攻撃者が任意のmintアカウントを渡せる可能性あり
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to:   ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.escrow_authority.to_account_info(),
        };
        transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
            amount,
        )?;

        // dataをまるごとゼロクリア（ownerチェックなし）
        ctx.accounts.listing_account.data.borrow_mut().fill(0);

        msg!(
            "Finalized sale for {}: transferred {} tokens",
            ctx.accounts.listing_account.key(),
            amount
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateListing<'info> {
    /// CHECK: owner == program_id の検証を全く行っていない
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,
    /// 出品者署名のみ検証
    pub seller: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct ExtendListing<'info> {
    /// CHECK: owner == program_id の検証を省略
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FinalizeSale<'info> {
    /// CHECK: Escrow用TokenAccountかどうか検証していない
    #[account(mut)]
    pub escrow_token_account: AccountInfo<'info>,
    /// CHECK: Seller用TokenAccountかどうか検証していない
    #[account(mut)]
    pub seller_token_account: AccountInfo<'info>,
    /// CHECK: Authorityの署名のみ検証
    pub escrow_authority: Signer<'info>,
    /// CHECK: owner == program_id チェックを行わない AccountInfo
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
}
