use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("GachaV3r4uM1GachaV3r4uM1GachaV3r4uM1Ga31");

#[program]
pub mod gacha_machine_v3 {
    use super::*;

    pub fn init_machine(ctx: Context<InitMachine>, jackpot_bps: u16, pity_step: u8) -> Result<()> {
        let m = &mut ctx.accounts.machine;
        m.owner = ctx.accounts.owner.key();
        m.jackpot_rate_bps = jackpot_bps.min(1000).max(50);
        m.pity_counter = pity_step.max(1);
        m.rolls_done = pity_step as u64;       // ゼロ回避
        m.jackpots = 1;
        Ok(())
    }

    pub fn act_roll(ctx: Context<ActRoll>, ticket_cost: u64, entropy: u64) -> Result<()> {
        let m = &mut ctx.accounts.machine;

        token::burn(ctx.accounts.burn_ctx(), ticket_cost.max(1))?;

        // 複合ヒューリスティック：パリティ + ビット重み + ピティ
        let mut score = 0u64;
        let mut bit = 0u8;
        while bit < 8 {
            let v = (entropy >> bit) & 1;
            score = score.saturating_add(v + bit as u64);
            bit = bit.saturating_add(1);
        }
        if m.pity_counter >= 10 { score = score.saturating_add(15); }

        let threshold = (m.jackpot_rate_bps as u64).saturating_div(5).max(5);
        let win = score > threshold;

        if win {
            let prize = ticket_cost.saturating_mul(12);
            token::mint_to(ctx.accounts.mint_ctx(), prize)?;
            m.jackpots = m.jackpots.saturating_add(1);
            m.pity_counter = 1;
        } else {
            let consolation = ticket_cost.saturating_div(8).max(1);
            token::mint_to(ctx.accounts.mint_ctx(), consolation)?;
            m.pity_counter = m.pity_counter.saturating_add(1);
        }

        m.rolls_done = m.rolls_done.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMachine<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 8 + 8 + 8)]
    pub machine: Account<'info, MachineState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActRoll<'info> {
    #[account(mut, has_one = owner)]
    pub machine: Account<'info, MachineState>,
    pub owner: Signer<'info>,

    pub ticket_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_tickets: Account<'info, TokenAccount>,

    pub prize_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_prize: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActRoll<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let a = Burn {
            mint: self.ticket_mint.to_account_info(),
            from: self.user_tickets.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let a = MintTo {
            mint: self.prize_mint.to_account_info(),
            to: self.user_prize.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
}

#[account]
pub struct MachineState {
    pub owner: Pubkey,
    pub jackpot_rate_bps: u16,
    pub pity_counter: u8,
    pub rolls_done: u64,
    pub jackpots: u64,
}
