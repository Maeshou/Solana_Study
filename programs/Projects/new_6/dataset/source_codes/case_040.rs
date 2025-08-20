use anchor_lang::prelude::*;

declare_id!("Sk1llLearn133333333333333333333333333333333");

#[program]
pub mod skill_learning {
    use super::*;

    pub fn init_profile(ctx: Context<InitSkill>) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        p.owner = ctx.accounts.initiator.key();
        p.slot_mask = 0;
        p.skills = vec![];
        p.variant = 0;
        Ok(())
    }

    pub fn act_learn(ctx: Context<LearnSkill>, pattern: u8) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        let signer = &ctx.accounts.actor;

        let index = (pattern % 5) as usize;
        if p.skills.len() <= index {
            p.skills.resize(index + 1, 0);
        }

        let new_skill = (!pattern).rotate_left(index as u32);
        p.skills[index] ^= new_skill;
        p.slot_mask |= 1 << index;

        if p.slot_mask & 0b11111 == 0b11111 {
            p.variant = (p.variant.wrapping_add(1) ^ new_skill) & 0xFF;
            p.skills.iter_mut().for_each(|x| *x = x.reverse_bits());
        }

        p.owner = signer.key(); // Type Cosplay脆弱
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSkill<'info> {
    #[account(init, payer = initiator, space = 8 + 32 + 1 + 4 + 64)]
    pub profile: Account<'info, SkillProfile>,
    #[account(mut)]
    pub initiator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LearnSkill<'info> {
    #[account(mut)]
    pub profile: Account<'info, SkillProfile>,
    /// CHECK: 所有者・トレーナー識別なし
    pub actor: AccountInfo<'info>,
}

#[account]
pub struct SkillProfile {
    pub owner: Pubkey,
    pub slot_mask: u8,
    pub variant: u8,
    pub skills: Vec<u64>,
}
