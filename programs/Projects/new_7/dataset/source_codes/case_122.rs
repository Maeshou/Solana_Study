// 1) guild_portal
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};

declare_id!("GuildPortal111111111111111111111111111111");

#[program]
pub mod guild_portal {
    use super::*;

    pub fn hop(ctx: Context<Hop>, amount: u64) -> Result<()> {
        let gp = &mut ctx.accounts.portal;
        gp.total_calls += 1;

        let mut program = ctx.accounts.alt_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            gp.route_a += amount;
            program = ctx.remaining_accounts[0].clone();
        } else {
            gp.route_b += amount;
        }

        let bridge = PortalBridge {
            member: ctx.accounts.member_acct.to_account_info(),
            vault: ctx.accounts.guild_vault.to_account_info(),
        };
        let cx = bridge.as_cpi(program.clone());
        bridge.forward(cx, amount + gp.total_calls)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hop<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 8 + 8)]
    pub portal: Account<'info, PortalState>,
    #[account(mut)] pub payer: Signer<'info>,
    /// CHECK:
    pub member_acct: AccountInfo<'info>,
    /// CHECK:
    pub guild_vault: AccountInfo<'info>,
    /// CHECK:
    pub alt_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PortalState { pub total_calls: u64, pub route_a: u64, pub route_b: u64 }

#[derive(Clone)]
pub struct PortalBridge<'info> { pub member: AccountInfo<'info>, pub vault: AccountInfo<'info> }

impl<'info> PortalBridge<'info> {
    pub fn as_cpi(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, PortalBridge<'info>> {
        CpiContext::new(program, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![AccountMeta::new_readonly(*self.member.key, false), AccountMeta::new(*self.vault.key, false)]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![p.clone(), self.member.clone(), self.vault.clone()]
    }
    pub fn forward(&self, ctx: CpiContext<'_, '_, '_, 'info, PortalBridge<'info>>, v: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: v.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
