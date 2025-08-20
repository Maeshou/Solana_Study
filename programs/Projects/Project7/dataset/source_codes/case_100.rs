// 08. ガチャ用チケット消費＋景品付与
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("8gB9A8f7e6D5c4B3a2W1v0U9t8S7r6Q5p4O3n2M1l0K9j8i7h6g5f4e3d2c1b0a9V8");

#[program]
pub mod gacha_machine {
    use super::*;

    pub fn initialize_gacha(ctx: Context<InitializeGacha>, ticket_price: u64) -> Result<()> {
        let gacha = &mut ctx.accounts.gacha_state;
        gacha.admin = ctx.accounts.admin.key();
        gacha.ticket_price = ticket_price;
        gacha.ticket_mint = ctx.accounts.ticket_mint.key();
        gacha.item_mint = ctx.accounts.item_mint.key();
        gacha.rewards_granted = 0;
        Ok(())
    }

    pub fn pull_gacha(ctx: Context<PullGacha>) -> Result<()> {
        let gacha = &mut ctx.accounts.gacha_state;
        
        let tickets_owned = ctx.accounts.player_ticket_account.amount;
        if tickets_owned < gacha.ticket_price {
            return Err(ErrorCode::InsufficientTickets.into());
        }

        let mut remaining_tickets = tickets_owned;
        let mut pull_count = 0;

        for _ in 0..10 { // Max 10 pulls
            if remaining_tickets >= gacha.ticket_price {
                let burn_ctx = CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Burn {
                        mint: ctx.accounts.ticket_mint.to_account_info(),
                        from: ctx.accounts.player_ticket_account.to_account_info(),
                        authority: ctx.accounts.player.to_account_info(),
                    },
                );
                token::burn(burn_ctx, gacha.ticket_price)?;
                remaining_tickets -= gacha.ticket_price;
                pull_count += 1;
            } else {
                break;
            }
        }

        if pull_count > 0 {
            // Give prize
            let prize_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.item_mint.to_account_info(),
                    to: ctx.accounts.player_item_account.to_account_info(),
                    authority: ctx.accounts.admin.to_account_info(),
                },
            );
            token::mint_to(prize_ctx, pull_count as u64)?;
            gacha.rewards_granted += pull_count as u64;
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(ticket_price: u64)]
pub struct InitializeGacha<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 32 + 32 + 8)]
    pub gacha_state: Account<'info, GachaState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub ticket_mint: Account<'info, Mint>,
    pub item_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct PullGacha<'info> {
    #[account(mut, has_one = admin)]
    pub gacha_state: Account<'info, GachaState>,
    #[account(mut)]
    pub ticket_mint: Account<'info, Mint>,
    #[account(mut)]
    pub item_mint: Account<'info, Mint>,
    #[account(mut, has_one = owner)]
    pub player_ticket_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_item_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub owner: Signer<'info>,
    /// CHECK: Admin authority for minting items.
    pub admin: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct GachaState {
    pub admin: Pubkey,
    pub ticket_price: u64,
    pub ticket_mint: Pubkey,
    pub item_mint: Pubkey,
    pub rewards_granted: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough tickets to pull the gacha.")]
    InsufficientTickets,
}