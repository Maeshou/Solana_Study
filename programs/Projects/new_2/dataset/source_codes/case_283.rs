// 10. バージョン管理（フラグで反転 or インクリメント）
use anchor_lang::prelude::*;

#[program]
pub mod version_controller {
    use super::*;
    pub fn bump(ctx: Context<Bump>, flag: bool) -> Result<()> {
        let buf = &mut ctx.accounts.ver.try_borrow_mut_data()?;
        if flag {
            buf[0] = !buf[0];
        } else {
            buf[0] = buf[0].wrapping_add(1);
        }
        msg!("管理者 {} が bump 実行", ctx.accounts.adm.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Bump<'info> {
    /// CHECK: 脆弱アカウント（検証なし）
    pub ver: AccountInfo<'info>,
    #[account(has_one = adm)]
    pub ver_ctrl: Account<'info, VersionAdmin>,
    pub adm: Signer<'info>,
}

#[account]
pub struct VersionAdmin { pub adm: Pubkey }
