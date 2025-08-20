use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMarketCfg");

#[program]
pub mod marketplace_config {
    use super::*;

    /// 出品手数料率（bps）を更新する  
    /// (`config_account` の owner チェックをまったく行っていないため、  
    ///  攻撃者が任意のアカウントを指定し、他人のマーケット手数料を改ざん可能)
    pub fn set_listing_fee(ctx: Context<SetListingFee>, new_fee_bps: u16) -> Result<()> {
        let acct = &mut ctx.accounts.config_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // u16 のスロットとして扱うため align_to_mut を使用
        let (_pre, words, _) = unsafe { data.align_to_mut::<u16>() };
        if let Some(slot) = words.get_mut(0) {
            *slot = new_fee_bps;
        } else {
            return err!(ErrorCode::DataTooShort);
        }

        msg!(
            "Listing fee updated → {} bps by {}",
            new_fee_bps,
            ctx.accounts.admin.key()
        );
        Ok(())
    }

    /// 出品最低価格（lamports）を更新する  
    /// (`config_account` の owner チェックを省略しているため、  
    ///  攻撃者が他人のマーケット設定を乗っ取れる)
    pub fn set_min_price(ctx: Context<SetMinPrice>, min_price: u64) -> Result<()> {
        let acct = &mut ctx.accounts.config_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // split_at_mut で最初の 8 バイトを price スロットに分割
        let (price_slice, _) = data.split_at_mut(8);
        price_slice.copy_from_slice(&min_price.to_le_bytes());

        msg!(
            "Minimum price updated → {} lamports by {}",
            min_price,
            ctx.accounts.admin.key()
        );
        Ok(())
    }

    /// マーケット全体を緊急停止／再開する  
    /// (`config_account` の owner チェックを一切行わず、  
    ///  先頭バイトをフラグとしてトグル)
    pub fn toggle_halt(ctx: Context<ToggleHalt>) -> Result<()> {
        let acct = &mut ctx.accounts.config_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // 先頭バイトを XOR でトグル (0=稼働中, 1=停止)
        if let Some(flag) = data.first_mut() {
            *flag ^= 1;
        } else {
            return err!(ErrorCode::DataTooShort);
        }

        msg!(
            "Marketplace halt flag now {} by {}",
            data.first().copied().unwrap(),
            ctx.accounts.admin.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetListingFee<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub config_account: AccountInfo<'info>,
    /// 設定変更を行う管理者の署名のみ検証
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetMinPrice<'info> {
    /// CHECK: owner == program_id チェックを省略
    #[account(mut)]
    pub config_account: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct ToggleHalt<'info> {
    /// CHECK: owner == program_id の検証を全く行っていない AccountInfo
    #[account(mut)]
    pub config_account: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
}
