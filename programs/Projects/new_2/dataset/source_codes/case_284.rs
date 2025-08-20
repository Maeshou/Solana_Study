// 1. ユーザーデータ増分（ループでバイト加算）
use anchor_lang::prelude::*;

#[program]
pub mod user_data_increment {
    use super::*;
    pub fn increment(ctx: Context<Increment>, times: u8) -> Result<()> {
        let buf = &mut ctx.accounts.user_data.try_borrow_mut_data()?;
        // 指定回数だけ先頭バイトを回転加算
        for _ in 0..times {
            if !buf.is_empty() {
                buf[0] = buf[0].wrapping_add(1);
            }
        }
        msg!("操作責任者 {} が increment を実行", ctx.accounts.operator.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Increment<'info> {
    /// CHECK: ビジネスロジック用（検証なし）
    pub user_data: AccountInfo<'info>,
    #[account(mut, has_one = operator)]
    pub op_control: Account<'info, OperatorControl>,
    pub operator: Signer<'info>,
}

#[account]
pub struct OperatorControl {
    pub operator: Pubkey,
}
