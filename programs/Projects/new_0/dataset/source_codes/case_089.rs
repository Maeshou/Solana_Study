use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfWHTLM");

#[program]
pub mod whitelist_manager {
    use super::*;

    /// ユーザーのホワイトリストレコードを初期化
    pub fn init_record(ctx: Context<InitRecord>) -> Result<()> {
        let rec = &mut ctx.accounts.record;
        rec.user        = ctx.accounts.user.key();
        rec.whitelisted = false;
        msg!("Initialized whitelist for {}", rec.user);
        Ok(())
    }

    /// 管理者がユーザーをホワイトリストに追加
    pub fn add_to_whitelist(ctx: Context<ModifyWhitelist>) -> Result<()> {
        require!(
            ctx.accounts.admin.is_signer,
            ErrorCode::Unauthorized
        );
        let rec = &mut ctx.accounts.record;
        rec.whitelisted = true;
        msg!("User {} whitelisted", rec.user);
        Ok(())
    }

    /// 管理者がユーザーをホワイトリストから削除
    pub fn remove_from_whitelist(ctx: Context<ModifyWhitelist>) -> Result<()> {
        require!(
            ctx.accounts.admin.is_signer,
            ErrorCode::Unauthorized
        );
        let rec = &mut ctx.accounts.record;
        rec.whitelisted = false;
        msg!("User {} removed from whitelist", rec.user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRecord<'info> {
    /// ホワイトリストレコードPDA
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 1,
        seeds = [b"white", user.key().as_ref()],
        bump
    )]
    pub record: Account<'info, WhitelistRecord>,

    /// 管理者署名
    #[account(mut)]
    pub admin: Signer<'info>,

    /// ホワイトリスト対象ユーザー
    pub user: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyWhitelist<'info> {
    /// 既存のホワイトリストレコード
    #[account(
        mut,
        seeds = [b"white", record.user.as_ref()],
        bump,
        has_one = user
    )]
    pub record: Account<'info, WhitelistRecord>,

    /// 管理者署名
    pub admin: Signer<'info>,

    /// 対象ユーザー
    pub user: UncheckedAccount<'info>,
}

#[account]
pub struct WhitelistRecord {
    /// ユーザーPubkey
    pub user:        Pubkey,
    /// ホワイトリストフラグ
    pub whitelisted: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: admin signature required")]
    Unauthorized,
}
