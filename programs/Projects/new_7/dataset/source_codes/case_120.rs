// 3) nft_relay_hub
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};

declare_id!("NftRelayHub111111111111111111111111111111");

#[program]
pub mod nft_relay_hub {
    use super::*;

    pub fn bounce(ctx: Context<Bounce>, serial: u64) -> Result<()> {
        let s = &mut ctx.accounts.hub;
        s.turns += 1;

        let mut program = ctx.accounts.default_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            s.path_a += serial;
            program = ctx.remaining_accounts[0].clone();
        } else {
            s.path_b += serial;
        }

        let b = NftBridge {
            holder: ctx.accounts.nft_holder.to_account_info(),
            vault: ctx.accounts.nft_vault.to_account_info(),
        };
        let cx = b.as_cpi(program.clone());
        b.fire(cx, serial + s.turns)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Bounce<'info> {
    #[account(init, payer = sender, space = 8 + 8 + 8 + 8)]
    pub hub: Account<'info, HubState>,
    #[account(mut)] pub sender: Signer<'info>,
    /// CHECK:
    pub nft_holder: AccountInfo<'info>,
    /// CHECK:
    pub nft_vault: AccountInfo<'info>,
    /// CHECK:
    pub default_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct HubState { pub turns: u64, pub path_a: u64, pub path_b: u64 }

#[derive(Clone)]
pub struct NftBridge<'info> { pub holder: AccountInfo<'info>, pub vault: AccountInfo<'info> }

impl<'info> NftBridge<'info> {
    pub fn as_cpi(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, NftBridge<'info>> {
        CpiContext::new(program, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![AccountMeta::new(*self.holder.key, false), AccountMeta::new(*self.vault.key, false)]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![p.clone(), self.holder.clone(), self.vault.clone()]
    }
    pub fn fire(&self, ctx: CpiContext<'_, '_, '_, 'info, NftBridge<'info>>, tag: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: tag.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
