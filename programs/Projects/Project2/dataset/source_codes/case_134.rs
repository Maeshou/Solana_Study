use anchor_lang::prelude::*;

declare_id!("Skil111111111111111111111111111111111111");

const MAX_ENDORSERS: usize = 50;

#[program]
pub mod skill_endorsment {
    /// 新しいスキルを登録
    pub fn create_skill(
        ctx: Context<CreateSkill>,
        name: String,
    ) -> Result<()> {
        // 名称長チェック
        if name.len() > 64 {
            return Err(ErrorCode::NameTooLong.into());
        }
        let skill = &mut ctx.accounts.skill;
        skill.owner      = ctx.accounts.user.key();  // Signer Authorization
        skill.name       = name;
        skill.count      = 0;
        skill.endorsers  = Vec::new();
        Ok(())
    }

    /// 他者のスキルをエンドース
    pub fn endorse_skill(ctx: Context<ModifySkill>) -> Result<()> {
        let skill = &mut ctx.accounts.skill;
        let user  = ctx.accounts.user.key();

        // 自己エンドース禁止
        if skill.owner == user {
            return Err(ErrorCode::CannotEndorseOwn.into());
        }
        // 上限チェック
        if skill.endorsers.len() >= MAX_ENDORSERS {
            return Err(ErrorCode::TooManyEndorsers.into());
        }
        // 重複チェック
        for &e in skill.endorsers.iter() {
            if e == user {
                return Err(ErrorCode::AlreadyEndorsed.into());
            }
        }
        // エンドース追加
        skill.endorsers.push(user);
        skill.count = skill.count.checked_add(1).unwrap();
        Ok(())
    }

    /// 自身のエンドースを取り消し
    pub fn remove_endorse(ctx: Context<ModifySkill>) -> Result<()> {
        let skill = &mut ctx.accounts.skill;
        let user  = ctx.accounts.user.key();

        // 探索
        let mut idx: Option<usize> = None;
        for (i, &e) in skill.endorsers.iter().enumerate() {
            if e == user {
                idx = Some(i);
                break;
            }
        }
        // 存在チェック・削除
        if let Some(i) = idx {
            skill.endorsers.remove(i);
            skill.count = skill.count.checked_sub(1).unwrap();
            Ok(())
        } else {
            Err(ErrorCode::NotEndorsed.into())
        }
    }
}

#[derive(Accounts)]
pub struct CreateSkill<'info> {
    /// 同一アカウント再初期化を防止
    #[account(init, payer = user, space = 8 + 32 + 4 + 64 + 4 + (MAX_ENDORSERS * 32))]
    pub skill:         Account<'info, SkillAccount>,

    /// 操作ユーザー（署名必須）
    #[account(mut)]
    pub user:          Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifySkill<'info> {
    /// 型チェック＆Owner Check
    #[account(mut)]
    pub skill: Account<'info, SkillAccount>,

    /// エンドース操作するユーザー（署名必須）
    pub user:  Signer<'info>,
}

#[account]
pub struct SkillAccount {
    /// スキル所有者
    pub owner:     Pubkey,
    /// スキル名称（最大64文字）
    pub name:      String,
    /// エンドース総数
    pub count:     u32,
    /// エンドーサーリスト（最大50名）
    pub endorsers: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("スキル名が長すぎます")]
    NameTooLong,
    #[msg("自分のスキルはエンドースできません")]
    CannotEndorseOwn,
    #[msg("すでにエンドース済みです")]
    AlreadyEndorsed,
    #[msg("エンドーサーが多すぎます")]
    TooManyEndorsers,
    #[msg("エンドースが見つかりません")]
    NotEndorsed,
}
