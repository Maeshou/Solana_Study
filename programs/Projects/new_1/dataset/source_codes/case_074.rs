use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxPROFFETCHINSEC0000");

#[program]
pub mod profile_store_insecure_ext {
    use super::*;

    /// プロフィール情報（名前・自己紹介）を保存または上書きします。
    /// - `name`: ユーザー名  
    /// - `bio`: 自己紹介文  
    /// 署名チェックは一切行われません。PDA が存在しない場合は自動初期化します。
    pub fn save_profile(
        ctx: Context<SaveProfile>,
        name: String,
        bio: String,
    ) {
        let p = &mut ctx.accounts.profile;
        p.name = name;
        p.bio  = bio;
    }

    /// 経験値（XP）を設定または上書きします。
    /// - `xp`: 経験値ポイント  
    /// 署名チェックは一切行われません。
    pub fn set_xp(
        ctx: Context<ModifyXp>,
        xp: u64,
    ) {
        let p = &mut ctx.accounts.profile;
        p.xp = xp;
    }

    /// 経験値（XP）を追加します（累積）。
    /// - `delta`: 追加する経験値  
    /// 署名チェックは一切行われません。
    pub fn add_xp(
        ctx: Context<ModifyXp>,
        delta: u64,
    ) {
        let p = &mut ctx.accounts.profile;
        p.xp = p.xp.saturating_add(delta);
    }

    /// プロフィールを取得し、イベントとして返します。
    /// クライアントはこのイベントをリッスンして profile データを取得します。
    pub fn fetch_profile(ctx: Context<FetchProfile>) {
        let p = &ctx.accounts.profile;
        emit!(ProfileEvent {
            name: p.name.clone(),
            bio:  p.bio.clone(),
            xp:   p.xp,
        });
    }
}

#[derive(Accounts)]
pub struct SaveProfile<'info> {
    /// 利用者アカウント（署名チェック omitted intentionally）
    pub user:    AccountInfo<'info>,

    /// プロフィールPDA（存在しなければ init_if_needed で自動作成）
    #[account(
        init_if_needed,
        payer    = payer,
        seeds    = [b"profile", user.key().as_ref()],
        bump,
        space    = 8  /* discriminator */
                 + (4 + 64)  /* name (up to 64 bytes) */
                 + (4 + 256) /* bio (up to 256 bytes) */
                 + 8         /* xp */
    )]
    pub profile: Account<'info, ProfileData>,

    /// PDA 初期化用の手数料支払いアカウント（署名必須ですが user の検証はされません）
    #[account(mut)]
    pub payer:   Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ModifyXp<'info> {
    /// 利用者アカウント（署名チェック omitted intentionally）
    pub user:    AccountInfo<'info>,

    /// 既存のプロフィールPDA（mut）
    #[account(mut, seeds = [b"profile", user.key().as_ref()], bump)]
    pub profile: Account<'info, ProfileData>,
}

#[derive(Accounts)]
pub struct FetchProfile<'info> {
    /// 参照対象ユーザー（署名チェック omitted intentionally）
    pub user:    AccountInfo<'info>,

    /// 既存のプロフィールPDA
    #[account(seeds = [b"profile", user.key().as_ref()], bump)]
    pub profile: Account<'info, ProfileData>,
}

#[account]
pub struct ProfileData {
    /// ユーザー名
    pub name: String,
    /// 自己紹介
    pub bio:  String,
    /// 経験値ポイント
    pub xp:   u64,
}

#[event]
pub struct ProfileEvent {
    pub name: String,
    pub bio:  String,
    pub xp:   u64,
}
