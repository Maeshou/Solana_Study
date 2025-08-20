use anchor_lang::prelude::*;
declare_id!("UserProfVuln111111111111111111111111111111");

/// ユーザープロフィール
#[account]
pub struct Profile {
    pub owner:   Pubkey, // プロフィールの所有者
    pub username: String,
    pub bio:     String,
}

/// プロフィール変更履歴
#[account]
pub struct ChangeRecord {
    pub editor:   Pubkey, // 変更を行ったユーザー
    pub profile:  Pubkey, // 本来は Profile.key() と一致すべき
    pub note:     String, // 変更内容のメモ
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 32 + 4 + 256)]
    pub profile:  Account<'info, Profile>,
    #[account(mut)]
    pub owner:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordChange<'info> {
    /// Profile.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub profile:  Account<'info, Profile>,

    /// ChangeRecord.profile ⇔ Profile.key() の検証がない
    #[account(init, payer = owner, space = 8 + 32 + 32 + 4 + 256)]
    pub record:   Account<'info, ChangeRecord>,

    #[account(mut)]
    pub owner:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBio<'info> {
    /// Profile.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub profile:  Account<'info, Profile>,

    /// ChangeRecord.profile ⇔ Profile.key() の検証がない
    #[account(mut)]
    pub record:   Account<'info, ChangeRecord>,

    pub owner:    Signer<'info>,
}

#[program]
pub mod profile_vuln {
    use super::*;

    /// プロフィールを作成
    pub fn create_profile(ctx: Context<CreateProfile>, username: String) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        p.owner    = ctx.accounts.owner.key();
        p.username = username;
        p.bio      = String::from("Welcome!");
        Ok(())
    }

    /// 変更履歴を記録
    pub fn record_change(ctx: Context<RecordChange>, note: String) -> Result<()> {
        let rec = &mut ctx.accounts.record;
        // 脆弱性ポイント：
        // rec.profile = ctx.accounts.profile.key(); とするだけで、
        // ChangeRecord.profile と Profile.key() の一致検証がない
        rec.editor  = ctx.accounts.owner.key();
        rec.profile = ctx.accounts.profile.key();
        rec.note    = note;
        Ok(())
    }

    /// プロフィールの自己紹介文（bio）を更新
    pub fn update_bio(ctx: Context<UpdateBio>, addition: String) -> Result<()> {
        let p   = &mut ctx.accounts.profile;
        let rec = &mut ctx.accounts.record;

        // 本来は必須：
        // require_keys_eq!(rec.profile, p.key(), ErrorCode::ProfileMismatch);

        // 変更履歴に基づき、bioの末尾に追記
        p.bio.reserve(addition.len());
        p.bio.push_str(" ");
        p.bio.push_str(&addition);
        // record.note を参照して、bio更新の理由をメタデータに反映
        rec.note = format!("Appended to bio: {}", addition);
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("ChangeRecord が指定の Profile と一致しません")]
    ProfileMismatch,
}
