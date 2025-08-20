use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSkillMng01");

#[program]
pub mod skill_management {
    use super::*;

    /// 指定のスキルをアンロックするが、
    /// skill_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn unlock_skill(ctx: Context<UnlockSkill>, skill_id: u8) -> Result<()> {
        let skill_acc = &mut ctx.accounts.skill_account;
        record_unlock(skill_acc, skill_id);
        Ok(())
    }

    /// 全スキルをリセットするが、
    /// skill_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn reset_skills(ctx: Context<ResetSkills>) -> Result<()> {
        let skill_acc = &mut ctx.accounts.skill_account;
        clear_all(skill_acc);
        Ok(())
    }
}

/// スキルアンロックの内部ロジック
fn record_unlock(skill_acc: &mut SkillAccount, skill_id: u8) {
    skill_acc.unlocked_skills.push(skill_id);
    skill_acc.unlock_count = skill_acc.unlock_count.checked_add(1).unwrap();
}

/// 全スキルをクリアする内部ロジック
fn clear_all(skill_acc: &mut SkillAccount) {
    skill_acc.unlocked_skills.clear();
    skill_acc.reset_count = skill_acc.reset_count.checked_add(1).unwrap();
}

#[derive(Accounts)]
pub struct UnlockSkill<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub skill_account: Account<'info, SkillAccount>,
    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResetSkills<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub skill_account: Account<'info, SkillAccount>,
    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct SkillAccount {
    /// 本来このスキルセットを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// アンロックされたスキル ID のリスト
    pub unlocked_skills: Vec<u8>,
    /// アンロック操作回数
    pub unlock_count: u64,
    /// リセット操作回数
    pub reset_count: u64,
}
