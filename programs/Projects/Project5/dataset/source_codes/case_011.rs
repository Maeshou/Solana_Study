// ============================================================================
// 1) RuneSmith（ルーン鍛造所）— PDA不使用 + constraint + has_one + require!
// ============================================================================
declare_id!("RNSM11111111111111111111111111111111");
use anchor_lang::prelude::*;

#[program]
pub mod runesmith {
    use super::*;

    pub fn init_smithy(ctx: Context<InitSmithy>, target_heat: u32) -> Result<()> {
        ctx.accounts.smithy.master = ctx.accounts.blacksmith.key();
        ctx.accounts.smithy.target_heat = target_heat;
        ctx.accounts.smithy.open = true;

        ctx.accounts.ingots.copper = 0;
        ctx.accounts.ingots.silver = 0;
        ctx.accounts.ingots.ready = true;

        ctx.accounts.ledger.makes = 0;
        ctx.accounts.ledger.fails = 0;
        Ok(())
    }

    pub fn forge_rune(ctx: Context<ForgeRune>, steps: u32) -> Result<()> {
        // 属性: anvil(=smithy) と ingots は異なる
        // 属性: smithy は has_one で master を固定
        // ここでは ledger との重複も防ぐ
        require!(ctx.accounts.ingots.key() != ctx.accounts.ledger.key(), SmithErr::Dup);

        let mut i = 0;
        while i < steps {
            ctx.accounts.ingots.copper = ctx.accounts.ingots.copper.saturating_add(1);
            ctx.accounts.ingots.silver = ctx.accounts.ingots.silver.saturating_add(1);
            ctx.accounts.ledger.makes = ctx.accounts.ledger.makes.saturating_add(1);
            i += 1;
        }

        if steps > ctx.accounts.smithy.target_heat {
            ctx.accounts.smithy.open = false;
            ctx.accounts.ingots.ready = false;
            ctx.accounts.ledger.fails = ctx.accounts.ledger.fails.saturating_add(1);
            msg!("overheat: smithy paused, fails={}", ctx.accounts.ledger.fails);
        } else {
            ctx.accounts.smithy.open = true;
            ctx.accounts.ingots.ready = true;
            ctx.accounts.ledger.makes = ctx.accounts.ledger.makes.saturating_add(1);
            msg!("good temper: makes={}", ctx.accounts.ledger.makes);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSmithy<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub smithy: Account<'info, Smithy>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 1)]
    pub ingots: Account<'info, Ingots>,
    #[account(init, payer = payer, space = 8 + 8 + 8)]
    pub ledger: Account<'info, ForgeLedger>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub blacksmith: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ForgeRune<'info> {
    #[account(mut, has_one = master)]
    pub smithy: Account<'info, Smithy>,
    #[account(mut, constraint = smithy.key() != ingots.key(), error = SmithErr::Dup)]
    pub ingots: Account<'info, Ingots>,
    #[account(mut)]
    pub ledger: Account<'info, ForgeLedger>,
    pub master: Signer<'info>,
}

#[account] pub struct Smithy { pub master: Pubkey, pub target_heat: u32, pub open: bool }
#[account] pub struct Ingots { pub copper: u32, pub silver: u32, pub ready: bool }
#[account] pub struct ForgeLedger { pub makes: u64, pub fails: u64 }

#[error_code] pub enum SmithErr { #[msg("duplicate mutable account")] Dup }

