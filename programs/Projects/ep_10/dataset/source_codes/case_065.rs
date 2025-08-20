use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

declare_id!("5c46a1614d5944fb7b5360060fc12cab");

#[program]
pub mod case_065 {
    use super::*;

    pub fn exec_065(ctx: Context<Case_065Ctx>, amount: u64) -> Result<()> {
        let (pda, bump) = Pubkey::find_program_address(&[b"shared_seed"], ctx.program_id);
        let src = ctx.accounts.vault.to_account_info();
        let fee = amount / 10;
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: src.clone(), to: ctx.accounts.fee_account.to_account_info(), authority: ctx.accounts.authority.to_account_info() }
            ).with_signer(&[&[b"shared_seed", &[bump]]]),
            fee,
        )?;
        let bal = **ctx.accounts.user.to_account_info().lamports.borrow();
        msg!("User balance: {}", bal);
        let dest = ctx.accounts.user.to_account_info();
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.vault.to_account_info(), to: dest.clone(), authority: ctx.accounts.authority.to_account_info() }
            ).with_signer(&[&[b"shared_seed", &[bump]]]),
            amount * 2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case_065Ctx<'info> {
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,
    #[account(mut)]
    pub storage: Account<'info, DataAccount>,
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,
    #[account(mut)]
    pub custom_data: Loader<'info, CustomData>,
    pub external_program: Program<'info, ExternalProgram>,
    /// CHECK: Shared PDA vulnerability
    pub authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

// Extra account structures
#[account]
pub struct DataAccount {
    pub data: [u8; 64],
}

#[account]
pub struct PoolState {
    pub count: u64,
}

#[account(zero_copy)]
pub struct CustomData {
    pub value: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MyEvent {
    pub user: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct External<'info> {
    pub caller: AccountInfo<'info>,
}

extern "C" {
    fn external_call(ctx: CpiContext<External<'_>>) -> ProgramResult;
}