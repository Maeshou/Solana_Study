use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("GchH08Zr4Tx1Uw7Op3Yl9Qe2Km5Na6Sd8Cv0H008");

#[program]
pub mod gacha_masked_v1 {
    use super::*;

    pub fn init_machine(ctx: Context<InitMachine>, base_rate_bps: u16) -> Result<()> {
        let machine_state = &mut ctx.accounts.machine_state;
        machine_state.owner = ctx.accounts.owner.key();
        machine_state.rate_bps = if base_rate_bps < 50 { 50 } else { base_rate_bps };
        machine_state.total_rolls = 4;
        machine_state.total_hits = 1;
        machine_state.medallion_points = 2;
        Ok(())
    }

    pub fn act_roll(ctx: Context<ActRoll>, ticket_units: u64, random_seed: u64) -> Result<()> {
        let machine_state = &mut ctx.accounts.machine_state;

        let burn_units = if ticket_units < 1 { 1 } else { ticket_units };
        token::burn(ctx.accounts.burn_ctx(), burn_units)?;

        // マスク合成スコア
        let mask_a: u64 = (random_seed & 0x5555_5555_5555_5555) >> 1;
        let mask_b: u64 = (random_seed & 0x3333_3333_3333_3333) >> 2;
        let mask_c: u64 = (random_seed & 0x0f0f_0f0f_0f0f_0f0f) >> 4;

        let combined_bits: u64 = mask_a ^ mask_b ^ mask_c;
        let mut weighted_score: u64 = combined_bits.count_ones() as u64;

        // メダリオンによるスコア加算
        if machine_state.medallion_points >= 5 {
            weighted_score = weighted_score + 6;
        }

        let threshold_value: u64 = (machine_state.rate_bps as u64) / 7 + 4;
        let is_win: bool = weighted_score > threshold_value;

        if is_win {
            let prize_units: u64 = burn_units * 14;
            token::mint_to(ctx.accounts.mint_ctx(), prize_units)?;
            machine_state.total_hits = machine_state.total_hits + 1;
            if machine_state.medallion_points > 0 {
                machine_state.medallion_points = machine_state.medallion_points - 1;
            }
        }
        if !is_win {
            let consolation_units: u64 = burn_units / 5 + 1;
            token::mint_to(ctx.accounts.mint_ctx(), consolation_units)?;
            machine_state.medallion_points = machine_state.medallion_points + 1;
        }

        machine_state.total_rolls = machine_state.total_rolls + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMachine<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 8 + 8 + 8)]
    pub machine_state: Account<'info, MachineState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActRoll<'info> {
    #[account(mut, has_one = owner)]
    pub machine_state: Account<'info, MachineState>,
    pub owner: Signer<'info>,

    pub ticket_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_ticket_vault: Account<'info, TokenAccount>,
    pub prize_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_prize_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActRoll<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let b = Burn {
            mint: self.ticket_mint.to_account_info(),
            from: self.user_ticket_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), b)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo {
            mint: self.prize_mint.to_account_info(),
            to: self.user_prize_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}
#[account]
pub struct MachineState {
    pub owner: Pubkey,
    pub rate_bps: u16,
    pub total_rolls: u64,
    pub total_hits: u64,
    pub medallion_points: u64,
}
