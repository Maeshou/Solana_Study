use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSkinSvc002");

#[program]
pub mod skin_service {
    use super::*;

    /// 指定したスキンを解除（アンロック）し、
    /// 対応するオーナーアカウントとの照合チェックがない
    pub fn unlock_skin(ctx: Context<UnlockSkin>, skin_id: u32) -> Result<()> {
        let skin_acc = &mut ctx.accounts.skin_account;
        let cost = ctx.accounts.config.skin_cost;

        // 1. ユーザーから料金（Lamports）を差し引き
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= cost;
        // 2. 運営口座へ料金を転送
        **ctx.accounts.treasury.to_account_info().lamports.borrow_mut() += cost;

        // 3. スキンIDを設定しアンロックフラグを立てる
        skin_acc.skin_id = skin_id;
        skin_acc.unlocked = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnlockSkin<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者一致を検証すべき
    pub skin_account: Account<'info, SkinAccount>,

    /// スキンをアンロックするユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// アンロック料金の受取先（運営口座）
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    /// スキンごとの料金設定を保持するアカウント
    pub config: Account<'info, SkinConfig>,
}

#[account]
pub struct SkinAccount {
    /// 本来このスキンを所有・操作できるユーザーの Pubkey
    pub owner: Pubkey,
    /// アンロックされたスキンのID
    pub skin_id: u32,
    /// アンロック済みフラグ
    pub unlocked: bool,
}

#[account]
pub struct SkinConfig {
    /// スキン1つあたりの解除料金（Lamports）
    pub skin_cost: u64,
}
