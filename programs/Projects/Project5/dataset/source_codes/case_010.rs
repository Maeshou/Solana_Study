// ============================================================================
// 2) Forge Smithy — 装備の鍛治（assert_ne!）
// ============================================================================
declare_id!("FOR42222222222222222222222222222222");

#[program]
pub mod forge_smithy {
    use super::*;

    pub fn init_forge(ctx: Context<InitForge>, heat: u32) -> Result<()> {
        ctx.accounts.anvil.durability = 100;
        ctx.accounts.anvil.owner = ctx.accounts.blacksmith.key();
        ctx.accounts.anvil.hot = false;

        ctx.accounts.materials.ore = 0;
        ctx.accounts.materials.spark = 0;
        ctx.accounts.materials.ready = true;

        ctx.accounts.rules.heat = heat;
        ctx.accounts.rules.cost = 3;
        ctx.accounts.rules.bump = *ctx.bumps.get("rules").unwrap();
        Ok(())
    }

    pub fn forge(ctx: Context<Forge>, steps: u32) -> Result<()> {
        // Duplicate 防止（anvil/materials、anvil/rules、materials/rules）
        assert_ne!(ctx.accounts.anvil.key(), ctx.accounts.materials.key(), "dup anvil/materials");
        assert_ne!(ctx.accounts.anvil.key(), ctx.accounts.rules.key(), "dup anvil/rules");
        assert_ne!(ctx.accounts.materials.key(), ctx.accounts.rules.key(), "dup materials/rules");

        for _ in 0..steps {
            ctx.accounts.anvil.durability = ctx.accounts.anvil.durability.saturating_sub(1);
            ctx.accounts.materials.ore = ctx.accounts.materials.ore.saturating_add(2);
            ctx.accounts.materials.spark = ctx.accounts.materials.spark.saturating_add(1);
        }

        if steps > ctx.accounts.rules.heat {
            ctx.accounts.anvil.hot = true;
            ctx.accounts.rules.cost = ctx.accounts.rules.cost.saturating_add(1);
            ctx.accounts.materials.ready = false;
            msg!("overheat; cost raised, materials cooling");
        } else {
            ctx.accounts.anvil.hot = false;
            ctx.accounts.rules.cost = ctx.accounts.rules.cost.saturating_add(0);
            ctx.accounts.materials.ready = true;
            msg!("temper ok; materials stable");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitForge<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub anvil: Account<'info, Anvil>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 1)]
    pub materials: Account<'info, MaterialBin>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 1, seeds = [b"rules", payer.key().as_ref()], bump)]
    pub rules: Account<'info, ForgeRules>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub blacksmith: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Forge<'info> {
    #[account(mut)]
    pub anvil: Account<'info, Anvil>,
    #[account(mut)]
    pub materials: Account<'info, MaterialBin>,
    #[account(mut)]
    pub rules: Account<'info, ForgeRules>,
}

#[account] pub struct Anvil { pub owner: Pubkey, pub durability: u32, pub hot: bool }
#[account] pub struct MaterialBin { pub ore: u32, pub spark: u32, pub ready: bool }
#[account] pub struct ForgeRules { pub heat: u32, pub cost: u32, pub bump: u8 }
