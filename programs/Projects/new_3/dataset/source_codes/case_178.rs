use anchor_lang::prelude::*;
declare_id!("SocialVuln1111111111111111111111111111111");

/// ユーザープロフィール
#[account]
pub struct Profile {
    pub owner:         Pubkey,   // プロフィール所有者
    pub followers:     u64,      // フォロワー数
    pub username:      String,
}

/// フォロー記録
#[account]
pub struct FollowRecord {
    pub follower:      Pubkey,   // フォロワー
    pub profile:       Pubkey,   // 本来は Profile.key() と一致すべき
    pub timestamp:     i64,      // フォローした時刻
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 4 + 32)]
    pub profile:       Account<'info, Profile>,
    #[account(mut)]
    pub owner:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Follow<'info> {
    /// FollowRecord.follower == follower.key() は検証される
    #[account(mut, has_one = follower)]
    pub record:        Account<'info, FollowRecord>,

    /// Profile.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub profile:       Account<'info, Profile>,

    pub follower:      Signer<'info>,
    pub owner:         Signer<'info>,
}

/// フォローを確定してカウンタ更新
#[derive(Accounts)]
pub struct ConfirmFollow<'info> {
    /// Profile.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub profile:       Account<'info, Profile>,

    /// FollowRecord.profile と Profile.key() の検証が **ない** 
    #[account(mut)]
    pub record:        Account<'info, FollowRecord>,

    pub owner:         Signer<'info>,
}

#[program]
pub mod social_vuln {
    use super::*;

    /// プロフィールを作成
    pub fn create_profile(ctx: Context<CreateProfile>, username: String) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        p.owner     = ctx.accounts.owner.key();
        p.username  = username;
        p.followers = 0;
        Ok(())
    }

    /// フォロー録を生成（初期化）
    pub fn follow(ctx: Context<Follow>, timestamp: i64) -> Result<()> {
        let r = &mut ctx.accounts.record;
        let p = &mut ctx.accounts.profile;
        // 脆弱性ポイント:
        // r.profile = p.key(); の一文もなく、
        // FollowRecord.profile と Profile.key() の一致検証がない
        r.follower  = ctx.accounts.follower.key();
        r.timestamp = timestamp;
        Ok(())
    }

    /// フォロー確定：フォロワー数を増やす
    pub fn confirm_follow(ctx: Context<ConfirmFollow>) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        // 本来は必須:
        // require_keys_eq!(ctx.accounts.record.profile, p.key(), ErrorCode::ProfileMismatch);

        // 直接加算を使ってフォロワー数を更新
        p.followers = p.followers + 1;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("FollowRecord が指定の Profile と一致しません")]
    ProfileMismatch,
}
