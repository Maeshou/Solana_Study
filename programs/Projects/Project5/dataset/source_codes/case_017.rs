// ============================================================================
// 2) Blade Foundry（刀剣鍛造）— PDA不使用 + constraint + has_one
//    防止法: has_one で所有者固定, 属性constraintで二重可変拒否（素材≠帳票, 鍛冶場≠素材）
// ============================================================================
declare_id!("BLDE22222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ForgeState { Closed, Open }

#[program]
pub mod blade_foundry {
    use super::*;

    pub fn init_foundry(ctx: Context<InitFoundry>, target: u32) -> Result<()> {
        ctx.accounts.foundry.owner = ctx.accounts.smith.key();
        ctx.accounts.foundry.target_heat = target;
        ctx.accounts.foundry.state = ForgeState::Open;

        ctx.accounts.materials.iron = 0;
        ctx.accounts.materials.coal = 0;
        ctx.accounts.materials.ready = 1; // 0:未,1:準備OK

        ctx.accounts.journal.success = 0;
        ctx.accounts.journal.failure = 0;
        Ok(())
    }

    pub fn temper(ctx: Context<Temper>, steps: u32) -> Result<()> {
        // ループ
        for _ in 0..steps {
            ctx.accounts.materials.iron = ctx.accounts.materials.iron.saturating_add(2);
            ctx.accounts.materials.coal = ctx.accounts.materials.coal.saturating_add(1);
            ctx.accounts.journal.success = ctx.accounts.journal.success.saturating_add(1);
        }
        // 分岐
        if steps > ctx.accounts.foundry.target_heat {
            ctx.accounts.foundry.state = ForgeState::Closed;
            ctx.accounts.materials.ready = 0;
            ctx.accounts.journal.failure = ctx.accounts.journal.failure.saturating_add(1);
            msg!("overheat; close foundry");
        } else {
            ctx.accounts.foundry.state = ForgeState::Open;
            ctx.accounts.materials.ready = 1;
            ctx.accounts.journal.success = ctx.accounts.journal.success.saturating_add(1);
            msg!("good temper; continue");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFoundry<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub foundry: Account<'info, Foundry>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 1)]
    pub materials: Account<'info, Materials>,
    #[account(init, payer = payer, space = 8 + 8 + 8)]
    pub journal: Account<'info, ForgeJournal>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub smith: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Temper<'info> {
    #[account(mut, has_one = owner, constraint = foundry.key() != materials.key(), error = ForgeErr::Same)]
    pub foundry: Account<'info, Foundry>,
    #[account(mut, constraint = materials.key() != journal.key(), error = ForgeErr::Same)]
    pub materials: Account<'info, Materials>,
    #[account(mut)]
    pub journal: Account<'info, ForgeJournal>,
    pub owner: Signer<'info>,
}

#[account] pub struct Foundry { pub owner: Pubkey, pub target_heat: u32, pub state: ForgeState }
#[account] pub struct Materials { pub iron: u32, pub coal: u32, pub ready: u8 }
#[account] pub struct ForgeJournal { pub success: u64, pub failure: u64 }

#[error_code] pub enum ForgeErr { #[msg("duplicate mutable account")] Same }
