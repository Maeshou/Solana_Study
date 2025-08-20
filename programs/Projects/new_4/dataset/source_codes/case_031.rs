// 4. バウチャー管理＋使用履歴
use anchor_lang::prelude::*;
declare_id!("VCHRAAAABBBBCCCCDDDDEEEEFFFF1111");

#[program]
pub mod misinit_voucher_v4 {
    use super::*;

    pub fn issue_voucher(ctx: Context<IssueVoucher>, code: String, expiry: i64) -> Result<()> {
        require!(expiry > Clock::get()?.unix_timestamp, ErrorCode3::InvalidExpiry);
        let vc = &mut ctx.accounts.voucher;
        vc.code = code;
        vc.valid_till = expiry;
        vc.redeemed = false;
        Ok(())
    }

    pub fn redeem_voucher(ctx: Context<IssueVoucher>, user: Pubkey) -> Result<()> {
        let vc = &mut ctx.accounts.voucher;
        require!(vc.valid_till > Clock::get()?.unix_timestamp, ErrorCode3::Expired);
        vc.redeemed = true;
        let log = &mut ctx.accounts.redemption_log;
        log.entries.push((user, Clock::get()?.unix_timestamp));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IssueVoucher<'info> {
    #[account(init, payer = authority, space = 8 + (4+32) + 8 + 1)] pub voucher: Account<'info, VoucherAccount>,
    #[account(mut)] pub redemption_log: Account<'info, RedemptionLog>,
    #[account(mut)] pub authority: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct VoucherAccount { pub code:String, pub valid_till:i64, pub redeemed:bool }
#[account]
pub struct RedemptionLog { pub entries: Vec<(Pubkey,i64)> }
#[error_code] pub enum ErrorCode3 { #[msg("期限が無効です。")]
    InvalidExpiry, #[msg("有効期限が切れています。")] Expired }
