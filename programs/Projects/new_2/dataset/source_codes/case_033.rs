use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqRoyalty01");

#[program]
pub mod nft_sale_distributor {
    use super::*;

    /// NFTの売上金を「販売者」と「ロイヤリティ受取先」に配分する  
    /// （`sale_account` の owner チェックを行っていないため、  
    ///  攻撃者が任意のアカウントを渡して売上内訳を改ざんできる脆弱性あり）
    pub fn distribute_sale(ctx: Context<DistributeSale>) -> Result<()> {
        // ── バイト列からロイヤリティ料率（bps）と売価（lamports）を抽出 ──
        let raw = ctx.accounts.sale_account.data.borrow();
        // 最初の 2 バイト：ロイヤリティ料率
        let (bps_bin, rest) = raw.split_at(2);
        // 続く 8 バイト：販売価格
        let (price_bin, _) = rest.split_at(8);
        let bps        = u16::from_le_bytes(bps_bin.try_into().unwrap());
        let sale_price = u64::from_le_bytes(price_bin.try_into().unwrap());

        // ── 料率に基づき配分額を計算 ──
        let royalty_amt = sale_price.saturating_mul(bps as u64) / 10_000;
        let seller_amt  = sale_price.saturating_sub(royalty_amt);

        // ── lamports を直接振り分け（owner チェックなし） ──
        let funds_acc = &mut ctx.accounts.sales_funds.to_account_info();
        let seller_acc = &mut ctx.accounts.seller.to_account_info();
        let royalty_acc = &mut ctx.accounts.royalty.to_account_info();

        **funds_acc.lamports.borrow_mut() -= sale_price;
        **seller_acc.lamports.borrow_mut() += seller_amt;
        **royalty_acc.lamports.borrow_mut() += royalty_amt;

        msg!(
            "Sale distributed → seller: {} lamports, royalty: {} lamports (bps={})",
            seller_amt,
            royalty_amt,
            bps
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributeSale<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない生の AccountInfo
    #[account(mut)]
    pub sale_account: AccountInfo<'info>,

    /// 売上金プール（lamports が引き出される）
    #[account(mut)]
    pub sales_funds: AccountInfo<'info>,

    /// NFT販売者の受取先アカウント
    #[account(mut)]
    pub seller: AccountInfo<'info>,

    /// ロイヤリティ受取先アカウント
    #[account(mut)]
    pub royalty: AccountInfo<'info>,

    /// 呼び出し元が署名していることのみ検証
    pub initiator: Signer<'info>,
}
