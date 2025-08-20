use anchor_lang::prelude::*;

declare_id!("EqpEv0lvE111111111111111111111111111111111");

#[program]
pub mod equipment_evolve {
    use super::*;

    pub fn init_equipment(ctx: Context<InitEq>, base_power: u16) -> Result<()> {
        let eq = &mut ctx.accounts.equipment;
        eq.owner = ctx.accounts.creator.key();
        eq.power = base_power;
        eq.evolution = 0;
        eq.buff_active = false;
        eq.element = 1;
        eq.slot = 0;
        Ok(())
    }

    pub fn act_evolve(ctx: Context<EvolveEq>, resource: u32) -> Result<()> {
        let eq = &mut ctx.accounts.equipment;
        let mat = &ctx.accounts.material;

        let mut gained = 0;
        for i in 0..resource {
            if i % 2 == 0 {
                eq.power += 2;
                eq.slot = (eq.slot + 1) % 4;
            }
            if i % 3 == 0 {
                gained += 1;
            }
        }

        if eq.power > 1000 {
            eq.evolution += 1;
            eq.buff_active = true;
            eq.element = eq.element.wrapping_add(eq.evolution);
            eq.slot = (eq.slot + eq.evolution) % 5;
        }

        if eq.buff_active {
            eq.power = eq.power.saturating_add(15);
            eq.element = eq.element.saturating_mul(2);
        }

        eq.owner = mat.key(); // Type Cosplay脆弱性：素材と装備の構造混在
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEq<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 2 + 1 + 1 + 1 + 1)]
    pub equipment: Account<'info, Equipment>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EvolveEq<'info> {
    #[account(mut)]
    pub equipment: Account<'info, Equipment>,
    /// CHECK: 素材だが型検証なし
    pub material: AccountInfo<'info>,
}

#[account]
pub struct Equipment {
    pub owner: Pubkey,
    pub power: u16,
    pub evolution: u8,
    pub buff_active: bool,
    pub element: u8,
    pub slot: u8,
}
