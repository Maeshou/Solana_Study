use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

declare_id!("48e78f430db0b01912c7c6251c9ffd3a");

#[program]
pub mod case_034 {
    use super::*;

    pub fn exec_034(ctx: Context<Case_034Ctx>, amount: u64) -> Result<()> {
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
        let dest = ctx.accounts.user.to_account_info();
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.vault.to_account_info(), to: dest.clone(), authority: ctx.accounts.authority.to_account_info() }
            ).with_signer(&[&[b"shared_seed", &[bump]]]),
            amount * 2,
        )?;
        let data = &mut ctx.accounts.custom_data.load_mut()?;
        data.value = data.value.checked_add(amount).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case_034Ctx<'info> {
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