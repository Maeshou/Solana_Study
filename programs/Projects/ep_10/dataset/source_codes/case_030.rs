use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

declare_id!("82c90a0d9d27b76ccbb0adc38d453bef");

#[program]
pub mod case_030 {
    use super::*;

    pub fn exec_030(ctx: Context<Case_030Ctx>, amount: u64) -> Result<()> {
        let (pda, bump) = Pubkey::find_program_address(&[b"shared_seed"], ctx.program_id);
        let data = &mut ctx.accounts.custom_data.load_mut()?;
        data.value = data.value.checked_add(amount).unwrap();
        let current = ctx.accounts.pool_state.count;
        ctx.accounts.pool_state.count = current + 1;
        let sol = ctx.accounts.system_program.to_account_info();
        invoke(&system_instruction::transfer(&ctx.accounts.authority.key(), &ctx.accounts.user.key(), amount), &[sol])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case_030Ctx<'info> {
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