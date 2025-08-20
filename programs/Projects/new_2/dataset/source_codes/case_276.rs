// 3. 設定トグル（バッファ長で分岐）
use anchor_lang::prelude::*;

#[program]
pub mod config_toggle {
    use super::*;
    pub fn toggle(ctx: Context<Toggle>) -> Result<()> {
        let buf = &mut ctx.accounts.settings_data.try_borrow_mut_data()?;
        if buf.len() >= 8 {
            buf[7] ^= 0xFF;
        } else {
            // 小さければすべてゼロ
            for b in buf.iter_mut() { *b = 0; }
        }
        msg!("責任者 {} が toggle を実行", ctx.accounts.responsible.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Toggle<'info> {
    /// CHECK: 脆弱アカウント（検証なし）
    pub settings_data: AccountInfo<'info>,
    #[account(mut, has_one = responsible)]
    pub config: Account<'info, ConfigAuthority>,
    pub responsible: Signer<'info>,
}

#[account]
pub struct ConfigAuthority { pub responsible: Pubkey }
