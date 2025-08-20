use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

declare_id!("0c7c928b9cc571f8dc04bf8b14a1e83d");

#[program]
pub mod case_009 {
    use super::*;

    pub fn exec_009(ctx: Context<Case_009Ctx>, amount: u64) -> Result<()> {
        let (pda, bump) = Pubkey::find_program_address(&[b"shared_seed"], ctx.program_id);
        let data = &mut ctx.accounts.custom_data.load_mut()?;
        data.value = data.value.checked_add(amount).unwrap();
        let mut info = ctx.accounts.storage.data.borrow_mut();
        info[0..8].copy_from_slice(&amount.to_le_bytes());
        let event = MyEvent { user: ctx.accounts.user.key(), amount };
        emit!(event);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case_009Ctx<'info> {
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