use anchor_lang::prelude::*;
use std::mem;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqTradeOffer");

#[program]
pub mod nft_trade_offer {
    use super::*;

    /// NFTトレードオファーを作成する  
    /// (`trade_account` の owner チェックを全く行っていないため、  
    /// 攻撃者が他人のオファーアカウントを指定して好き勝手に
    /// 提案内容を上書きできる脆弱性があります)
    pub fn create_offer(
        ctx: Context<CreateOffer>,
        offered_nfts: Vec<Pubkey>,   // 提供するNFTリスト
        wanted_nfts:  Vec<Pubkey>,   // 要求するNFTリスト
    ) -> Result<()> {
        let acct = &mut ctx.accounts.trade_account.to_account_info();
        let buf  = &mut acct.data.borrow_mut();

        // ── レイアウト想定 ──
        // [u8 offered_count][offered_count×32 bytes]
        // [u8 wanted_count ][wanted_count×32 bytes]
        // [32 bytes proposer pubkey]

        // 各リストの長さキャップ
        let o = offered_nfts.len().min(8) as u8;
        let w = wanted_nfts.len().min(8) as u8;
        let mut record = Vec::with_capacity(1 + 32 * (o as usize) + 1 + 32 * (w as usize) + 32);
        record.push(o);
        for &pk in offered_nfts.iter().take(o as usize) {
            record.extend_from_slice(&pk.to_bytes());
        }
        record.push(w);
        for &pk in wanted_nfts.iter().take(w as usize) {
            record.extend_from_slice(&pk.to_bytes());
        }
        record.extend_from_slice(ctx.accounts.proposer.key.as_ref());

        // バッファ書き込み
        if buf.len() < record.len() {
            return err!(ErrorCode::DataTooShort);
        }
        buf.iter_mut()
            .zip(record.iter())
            .for_each(|(dst, &src)| *dst = src);

        msg!(
            "Offer {} created by {}: offers {} items, wants {} items",
            acct.key(),
            ctx.accounts.proposer.key(),
            o,
            w
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOffer<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub trade_account: AccountInfo<'info>,
    /// オファー提案者の署名のみ検証
    pub proposer:      Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("データ領域が不足しているためオファーを保存できません")]
    DataTooShort,
}
