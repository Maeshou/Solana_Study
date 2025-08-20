// 8. 手数料管理モジュール
use anchor_lang::prelude::*;

#[program]
pub mod fee_manager {
    use super::*;
    // プールをモードに応じてクリア
    pub fn purge(ctx: Context<Purge>, mode: u8) -> Result<()> {
        let buf = &mut ctx.accounts.fee_pool.try_borrow_mut_data()?;
        let mid = buf.len() / 2;
        if mode == 0 {
            for i in 0..mid { buf[i] = 0; }
        } else {
            for i in mid..buf.len() { buf[i] = 0; }
        }
        msg!("会計 {} が purge (mode={})", ctx.accounts.accountant.key(), mode);
        Ok(())
    }
    // クリア済みプールを復元（全体に1書き込み）
    pub fn restore_fees(ctx: Context<RestoreFees>) -> Result<()> {
        let buf = &mut ctx.accounts.fee_pool.try_borrow_mut_data()?;
        for b in buf.iter_mut() { *b = 1; }
        msg!("会計 {} が復元実行", ctx.accounts.accountant.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Purge<'info> {
    /// CHECK: 手数料プール（検証なし）
    pub fee_pool: AccountInfo<'info>,
    #[account(has_one = accountant)]
    pub fee_ctrl: Account<'info, FeeControl>,
    pub accountant: Signer<'info>,
}

#[derive(Accounts)]
pub struct RestoreFees<'info> {
    /// CHECK: 手数料プール（検証なし）
    pub fee_pool: AccountInfo<'info>,
    #[account(mut, has_one = accountant)]
    pub fee_ctrl: Account<'info, FeeControl>,
    pub accountant: Signer<'info>,
}

#[account]
pub struct FeeControl {
    pub accountant: Pubkey,
}
