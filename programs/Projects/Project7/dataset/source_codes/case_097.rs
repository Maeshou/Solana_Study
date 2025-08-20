// 06. シーズンパスのチャージ
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("8gA7z6F5e4D3c2B1a0Wv9U8t7S6R5Q4P3O2N1M0L9K8J7I6H5G4F3E2D1C0B9A8f7");

#[program]
pub mod season_pass_manager {
    use super::*;

    pub fn initialize_pass_state(ctx: Context<InitializePassState>, max_charges: u32, charge_token_amount: u64) -> Result<()> {
        let pass_state = &mut ctx.accounts.pass_state;
        pass_state.owner = ctx.accounts.owner.key();
        pass_state.charges_left = max_charges;
        pass_state.max_charges = max_charges;
        pass_state.charge_token_amount = charge_token_amount;
        pass_state.charge_token_mint = ctx.accounts.charge_token_mint.key();
        Ok(())
    }

    pub fn charge_pass(ctx: Context<ChargePass>) -> Result<()> {
        let pass_state = &mut ctx.accounts.pass_state;
        if pass_state.charges_left == 0 {
            return Err(ErrorCode::PassFullyCharged.into());
        }

        let mut charges_added = 0;
        let mut loop_counter = 0;
        while loop_counter < 5 { // Max 5 charges at once
            if pass_state.charges_left > 0 {
                let charge_ctx = CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.owner_token_account.to_account_info(),
                        to: ctx.accounts.pass_token_account.to_account_info(),
                        authority: ctx.accounts.owner.to_account_info(),
                    },
                );
                token::transfer(charge_ctx, pass_state.charge_token_amount)?;
                pass_state.charges_left -= 1;
                charges_added += 1;
            } else {
                break;
            }
            loop_counter += 1;
        }

        if charges_added == 0 {
            return Err(ErrorCode::PassFullyCharged.into());
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(max_charges: u32, charge_token_amount: u64)]
pub struct InitializePassState<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4 + 8 + 32)]
    pub pass_state: Account<'info, SeasonPassState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub charge_token_mint: Account<'info, Mint>,
    #[account(init, payer = owner, token::mint = charge_token_mint, token::authority = owner)]
    pub pass_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ChargePass<'info> {
    #[account(mut, has_one = owner)]
    pub pass_state: Account<'info, SeasonPassState>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pass_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct SeasonPassState {
    pub owner: Pubkey,
    pub charges_left: u32,
    pub max_charges: u32,
    pub charge_token_amount: u64,
    pub charge_token_mint: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Season pass is already fully charged.")]
    PassFullyCharged,
}