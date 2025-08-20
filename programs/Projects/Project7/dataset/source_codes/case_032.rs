use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("GachaV4n7Qk2Xw5De8Rt1AaBbCcDdEeFfGgHhIiJj008");

#[program]
pub mod gacha_v4 {
    use super::*;

    pub fn init_machine(ctx: Context<InitMachine>, jackpot_bps: u16, pity_increment: u8) -> Result<()> {
        let machine = &mut ctx.accounts.machine;
        machine.owner_key = ctx.accounts.owner.key();
        machine.jackpot_bps = jackpot_bps.min(1000).max(50);
        machine.pity_counter = pity_increment.max(1);
        machine.roll_count = (pity_increment as u64).saturating_add(2);
        machine.jackpot_hits = 2;
        Ok(())
    }

    pub fn act_roll(ctx: Context<ActRoll>, ticket_cost: u64, entropy_seed: u64) -> Result<()> {
        let machine = &mut ctx.accounts.machine;

        token::burn(ctx.accounts.burn_ctx(), ticket_cost.max(1))?;

        // 重み付きスコア（下位ビットほど重み小）
        let mut weighted_score = 0u64;
        let mut bit_index = 0u8;
        while bit_index < 12 {
            let bit = (entropy_seed >> bit_index) & 1;
            let weight = (12 - bit_index as u64);
            weighted_score = weighted_score.saturating_add(bit.saturating_mul(weight));
            bit_index = bit_index.saturating_add(1);
        }
        if machine.pity_counter >= 8 { weighted_score = weighted_score.saturating_add(10); }

        let threshold = (machine.jackpot_bps as u64).saturating_div(6).max(4);
        let win_flag = weighted_score > threshold;

        if win_flag {
            let prize_units = ticket_cost.saturating_mul(15);
            token::mint_to(ctx.accounts.mint_ctx(), prize_units)?;
            machine.jackpot_hits = machine.jackpot_hits.saturating_add(1);
            machine.pity_counter = 1;
        } else {
            let consolation = ticket_cost.saturating_div(6).max(1);
            token::mint_to(ctx.accounts.mint_ctx(), consolation)?;
            machine.pity_counter = machine.pity_counter.saturating_add(1);
        }

        machine.roll_count = machine.roll_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMachine<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 1 + 8 + 8)]
    pub machine: Account<'info, MachineState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActRoll<'info> {
    #[account(mut, has_one = owner_key)]
    pub machine: Account<'info, MachineState>,
    pub owner_key: Signer<'info>,

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
            authority: self.owner_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), b)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo {
            mint: self.prize_mint.to_account_info(),
            to: self.user_prize_vault.to_account_info(),
            authority: self.owner_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}

#[account]
pub struct MachineState {
    pub owner_key: Pubkey,
    pub jackpot_bps: u16,
    pub pity_counter: u8,
    pub roll_count: u64,
    pub jackpot_hits: u64,
}
