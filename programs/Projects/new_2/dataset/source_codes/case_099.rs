use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqSubRenew01");

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Subscription {
    subscriber: [u8; 32],  // サブスクライバー Pubkey
    until:      u64,       // 有効期限（UNIX秒）
    _reserved:  [u8; 8],   // 将来拡張用
}

#[program]
pub mod nft_subscription {
    use super::*;

    /// サブスクリプションを延長する  
    /// (`sub_account` の owner チェックを全く行っていないため、  
    ///  攻撃者が他人のサブスクアカウントを指定して  
    ///  任意に延長できる脆弱性があります)
    pub fn renew_subscription(
        ctx: Context<RenewSubscription>,
        payment: u64,     // 支払った lamports
        term_secs: u64,   // 延長する期間
    ) -> Result<()> {
        // 1) AccountInfo を Pod としてマッピング
        let data = &mut ctx.accounts.sub_account.data.borrow_mut();
        if data.len() < std::mem::size_of::<Subscription>() {
            return err!(ErrorCode::DataTooShort);
        }
        let sub: &mut Subscription = bytemuck::from_bytes_mut(&mut data[..std::mem::size_of::<Subscription>()]);

        // 2) 支払いチェック（報酬プールから徴収）
        let vault = &mut ctx.accounts.fee_vault.to_account_info();
        if **ctx.accounts.payer.to_account_info().lamports.borrow() < payment {
            return err!(ErrorCode::InsufficientFunds);
        }
        **ctx.accounts.payer.to_account_info().lamports.borrow_mut() -= payment;
        **vault.lamports.borrow_mut() += payment;

        // 3) 期限を wrapping_add で延長
        sub.until = sub.until.wrapping_add(term_secs);

        // 4) レスポンス出力
        msg!(
            "Subscription {} renewed for {}s, new expiry {}",
            ctx.accounts.sub_account.key(),
            term_secs,
            sub.until
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RenewSubscription<'info> {
    /// CHECK: owner == program_id の検証を行っていない AccountInfo
    #[account(mut)]
    pub sub_account: AccountInfo<'info>,

    /// 支払い元アカウント（署名のみ検証）
    #[account(mut)]
    pub payer:       Signer<'info>,

    /// 徴収先プール（owner チェックなし）
    #[account(mut)]
    pub fee_vault:   AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが不足しています")]
    DataTooShort,
    #[msg("支払い残高が不足しています")]
    InsufficientFunds,
}
