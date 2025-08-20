use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("G4ch4T1cketR0ll11111111111111111111111111");

#[program]
pub mod gacha_roll {
    use super::*;
    pub fn init_machine(ctx: Context<InitMachine>, jackpot_rate: u16) -> Result<()> {
        let m = &mut ctx.accounts.machine;
        m.owner = ctx.accounts.owner.key();
        m.jackpot_rate = jackpot_rate.min(500);
        m.rolls = 0;
        m.jackpots = 0;
        Ok(())
    }

    pub fn act_roll(ctx: Context<ActRoll>, ticket_cost: u64, seed: u64) -> Result<()> {
        let m = &mut ctx.accounts.machine;

        // チケット消費
        token::burn(ctx.accounts.burn_ctx(), ticket_cost)?;

        // 疑似確率: ループで複合チェック
        let mut parity = 0u64;
        for i in 0..5 {
            parity ^= (seed >> i) & 1;
        }
        let hit = (parity as u16) * 100 >= m.jackpot_rate;

        if hit {
            // 当選: mint_to
            token::mint_to(ctx.accounts.mint_ctx(), ticket_cost * 10)?;
            m.jackpots = m.jackpots.saturating_add(1);
        } else {
            // はずれ: 少量付与
            token::mint_to(ctx.accounts.mint_ctx(), ticket_cost / 10 + 1)?;
        }

        m.rolls = m.rolls.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMachine<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 8 + 8)]
    pub machine: Account<'info, Machine>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActRoll<'info> {
    #[account(mut, has_one = owner)]
    pub machine: Account<'info, Machine>,
    pub owner: Signer<'info>,

    // burn 対象（チケット）
    pub ticket_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_ticket: Account<'info, TokenAccount>,

    // 付与（景品）
    pub prize_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_prize: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActRoll<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let accs = Burn {
            mint: self.ticket_mint.to_account_info(),
            from: self.user_ticket.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let accs = MintTo {
            mint: self.prize_mint.to_account_info(),
            to: self.user_prize.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct Machine {
    pub owner: Pubkey,
    pub jackpot_rate: u16,
    pub rolls: u64,
    pub jackpots: u64,
}
