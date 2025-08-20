// 8. 初期化（シード値で分岐）
use anchor_lang::prelude::*;

#[program]
pub mod initializer {
    use super::*;
    pub fn init(ctx: Context<Init>, seed: u8) -> Result<()> {
        let buf = &mut ctx.accounts.data.try_borrow_mut_data()?;
        if seed % 2 == 0 {
            buf[0] = seed;
        } else {
            buf[0] = seed.wrapping_mul(2);
        }
        msg!("認可者 {} が初期化実行", ctx.accounts.auth.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    /// CHECK: 脆弱アカウント（検証なし）
    pub data: AccountInfo<'info>,
    #[account(has_one = auth)]
    pub init_ctrl: Account<'info, InitAdmin>,
    pub auth: Signer<'info>,
}

#[account]
pub struct InitAdmin { pub auth: Pubkey }
