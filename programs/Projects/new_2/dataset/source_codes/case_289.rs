// 9. 手数料計算（for ループ＋単純加算）
use anchor_lang::prelude::*;

#[program]
pub mod fee_calculator {
    use super::*;
    pub fn calculate(ctx: Context<Calculate>, txs: u8) -> Result<()> {
        let buf = &mut ctx.accounts.fee_pool.try_borrow_mut_data()?;
        for _ in 0..txs {
            if !buf.is_empty() {
                buf[0] = buf[0].wrapping_add(2);
            }
        }
        msg!("計算責任者 {} が calculate を実行", ctx.accounts.accountant.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Calculate<'info> {
    /// CHECK: 手数料プール用（検証なし）
    pub fee_pool: AccountInfo<'info>,
    #[account(has_one = accountant)]
    pub fee_ctrl: Account<'info, FeeControl>,
    pub accountant: Signer<'info>,
}

#[account]
pub struct FeeControl {
    pub accountant: Pubkey,
}
