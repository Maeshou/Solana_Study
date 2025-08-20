
// 4. バウチャー管理＋使用履歴（Clockなし）
use anchor_lang::prelude::*;
declare_id!("VCHRBARBARBARBARBARBARBARBAR8888");

#[program]
pub mod misinit_voucher_no_clock {
    use super::*;

    pub fn issue_voucher(
        ctx: Context<IssueVoucher>,
        code: String,
        max_uses: u8,
    ) -> Result<()> {
        let vc = &mut ctx.accounts.voucher;
        vc.code = code;
        vc.remaining = max_uses;
        Ok(())
    }

    pub fn redeem_voucher(
        ctx: Context<IssueVoucher>,
        user: Pubkey,
    ) -> Result<()> {
        let vc = &mut ctx.accounts.voucher;
        require!(vc.remaining > 0, ErrorCode4::NoUses);
        vc.remaining = vc.remaining.checked_sub(1).unwrap();
        let log = &mut ctx.accounts.redemption_log;
        if log.records.len() >= 10 { log.records.remove(0); }
        log.records.push((user, vc.remaining));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IssueVoucher<'info> {
    #[account(init, payer = authority, space = 8 + (4+32) + 1)] pub voucher: Account<'info, VoucherAcct>,
    #[account(mut)] pub redemption_log: Account<'info, RedemptionLog>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VoucherAcct { pub code:String, pub remaining:u8 }
#[account]
pub struct RedemptionLog { pub records: Vec<(Pubkey,u8)> }

#[error_code]
pub enum ErrorCode4 { #[msg("使用回数がありません。")]
    NoUses }