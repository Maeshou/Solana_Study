// ============================================================================
// 6) Guild Atelier（ギルド工房）— PDA不使用 + has_one + constraint（二重可変の交差禁止）
//    防止法: owner固定と、member_a/member_b/treasuryの相互≠を属性で保証
// ============================================================================
declare_id!("ATLR66666666666666666666666666666666");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum WorkshopState { Locked, Open }

#[program]
pub mod guild_atelier {
    use super::*;

    pub fn init_atelier(ctx: Context<InitAtelier>, cap: u32) -> Result<()> {
        ctx.accounts.workshop.leader = ctx.accounts.leader.key();
        ctx.accounts.workshop.capacity = cap;
        ctx.accounts.workshop.state = WorkshopState::Open;

        ctx.accounts.member_a.user = ctx.accounts.user_a.key();
        ctx.accounts.member_a.skill = 1;
        ctx.accounts.member_a.active = 1;

        ctx.accounts.member_b.user = ctx.accounts.user_b.key();
        ctx.accounts.member_b.skill = 1;
        ctx.accounts.member_b.active = 1;

        ctx.accounts.treasury.coins = 0;
        ctx.accounts.treasury.locked = 0;
        Ok(())
    }

    pub fn craft(ctx: Context<Craft>, steps: u32) -> Result<()> {
        // ループ
        let mut i = 0;
        while i < steps {
            ctx.accounts.member_a.skill = ctx.accounts.member_a.skill.saturating_add(1);
            ctx.accounts.member_b.skill = ctx.accounts.member_b.skill.saturating_add(1);
            ctx.accounts.treasury.coins = ctx.accounts.treasury.coins.saturating_add(2);
            i += 1;
        }

        // 分岐
        if (ctx.accounts.member_a.skill + ctx.accounts.member_b.skill) > ctx.accounts.workshop.capacity {
            ctx.accounts.workshop.state = WorkshopState::Locked;
            ctx.accounts.treasury.locked = 1;
            ctx.accounts.treasury.coins = ctx.accounts.treasury.coins.saturating_add(5);
            msg!("capacity hit; lock treasury");
        } else {
            ctx.accounts.workshop.state = WorkshopState::Open;
            ctx.accounts.treasury.locked = 0;
            ctx.accounts.treasury.coins = ctx.accounts.treasury.coins.saturating_add(1);
            msg!("room remains; mint small bonus");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAtelier<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub workshop: Account<'info, Workshop>,
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub member_a: Account<'info, Member>,
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub member_b: Account<'info, Member>,
    #[account(init, payer = payer, space = 8 + 8 + 1)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub leader: Signer<'info>,
    pub user_a: Signer<'info>,
    pub user_b: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Craft<'info> {
    #[account(mut, has_one = leader)]
    pub workshop: Account<'info, Workshop>,
    #[account(mut, constraint = member_a.key() != member_b.key(), error = AtlErr::Same, constraint = member_a.key() != treasury.key(), error = AtlErr::Same)]
    pub member_a: Account<'info, Member>,
    #[account(mut, constraint = member_b.key() != treasury.key(), error = AtlErr::Same)]
    pub member_b: Account<'info, Member>,
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    pub leader: Signer<'info>,
}

#[account] pub struct Workshop { pub leader: Pubkey, pub capacity: u32, pub state: WorkshopState }
#[account] pub struct Member { pub user: Pubkey, pub skill: u32, pub active: u8 }
#[account] pub struct Treasury { pub coins: u64, pub locked: u8 }

#[error_code] pub enum AtlErr { #[msg("dup")] Same }