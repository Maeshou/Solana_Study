use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSkillAlloc01");

#[program]
pub mod skill_allocation {
    use super::*;

    /// ユーザーにスキルポイントを割り当てるが、
    /// skill_account.owner と ctx.accounts.user.key() の照合チェックを行っていない
    pub fn allocate_skill_points(
        ctx: Context<AllocateSkillPoints>,
        strength: u8,
        agility: u8,
        intelligence: u8,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.skill_account;

        // 1. 各スキルにポイントを加算
        acct.strength = acct.strength.checked_add(strength as u64).unwrap();
        acct.agility = acct.agility.checked_add(agility as u64).unwrap();
        acct.intelligence = acct.intelligence.checked_add(intelligence as u64).unwrap();

        // 2. 総使用ポイント数を更新
        acct.points_allocated = acct.points_allocated
            .checked_add(strength as u64 + agility as u64 + intelligence as u64)
            .unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AllocateSkillPoints<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub skill_account: Account<'info, SkillAccount>,

    /// スキル割り当てを行うユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct SkillAccount {
    /// このスキルアカウントを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 力（Strength）
    pub strength: u64,
    /// 敏捷性（Agility）
    pub agility: u64,
    /// 知力（Intelligence）
    pub intelligence: u64,
    /// 割り当て済みスキルポイントの総数
    pub points_allocated: u64,
}
