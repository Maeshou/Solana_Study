// 1) DualMintOrTransfer: mint_to（Token）と transfer（System）をフラグで切替
use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as SysTransfer};
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

declare_id!("DualMintOrTransfer11111111111111111111111");

#[program]
pub mod dual_mint_or_transfer {
    use super::*;
    pub fn initialize(ctx: Context<InitializeDual>, mint_amount: u64, sol_amount: u64, pay_in_sol: bool) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        settings.admin = ctx.accounts.admin.key();
        settings.mint_amount = mint_amount.max(1);
        settings.sol_amount = sol_amount;
        settings.pay_in_sol = pay_in_sol;
        Ok(())
    }
    pub fn execute(ctx: Context<ExecuteDual>) -> Result<()> {
        if ctx.accounts.settings.pay_in_sol {
            system_program::transfer(ctx.accounts.system_ctx(), ctx.accounts.settings.sol_amount)?;
        }
        if !ctx.accounts.settings.pay_in_sol {
            token::mint_to(ctx.accounts.mint_ctx(), ctx.accounts.settings.mint_amount)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDual<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 1)]
    pub settings: Account<'info, DualSettings>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ExecuteDual<'info> {
    #[account(mut, has_one = admin)]
    pub settings: Account<'info, DualSettings>,
    pub admin: Signer<'info>,
    // System transfer
    #[account(mut)] pub sol_payer: Signer<'info>,
    #[account(mut)] pub sol_receiver: Account<'info, SystemAccount>,
    pub system_program: Program<'info, System>,
    // Token mint_to
    #[account(mut)] pub reward_mint: Account<'info, Mint>,
    #[account(mut)] pub reward_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> ExecuteDual<'info> {
    fn system_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SysTransfer<'info>> {
        CpiContext::new(self.system_program.to_account_info(), SysTransfer {
            from: self.sol_payer.to_account_info(),
            to: self.sol_receiver.to_account_info(),
        })
    }
    fn mint_ctx(&self) -> CpiContext<'_, '_, '_', 'info, MintTo<'info>> {
        CpiContext::new(self.token_program.to_account_info(), MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.reward_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        })
    }
}
#[account] pub struct DualSettings { pub admin: Pubkey, pub mint_amount: u64, pub sol_amount: u64, pub pay_in_sol: bool }
