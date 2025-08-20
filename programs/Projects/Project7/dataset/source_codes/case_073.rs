use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Rebate7EntryA9uXk2Wm4Qy6Vt8Rb0Lc3Za5Hd7Q307");

#[program]
pub mod entry_fee_rebate_v1 {
    use super::*;

    pub fn init_event(ctx: Context<InitEvent>, base_fee_input: u64, cap_per_round_input: u64) -> Result<()> {
        let event = &mut ctx.accounts.event_state;
        event.organizer = ctx.accounts.organizer.key();
        event.base_fee = base_fee_input;
        if event.base_fee < 1 { event.base_fee = 1; }
        event.cap_per_round = cap_per_round_input;
        if event.cap_per_round < event.base_fee { event.cap_per_round = event.base_fee; }
        event.issued_in_round = 0;
        event.first_bonus_issued = 0;
        Ok(())
    }

    pub fn act_rebate(ctx: Context<ActRebate>, entrants_count: u16, is_early_bird: bool) -> Result<()> {
        let event = &mut ctx.accounts.event_state;

        let mut rebate_units: u64 = event.base_fee / 5 + 1;
        let mut entrant_cursor: u16 = 1;
        while entrant_cursor < entrants_count {
            rebate_units = rebate_units + (entrant_cursor as u64 % 5);
            entrant_cursor = entrant_cursor + 1;
        }

        if is_early_bird { rebate_units = rebate_units + 2; }

        let projected = event.issued_in_round + rebate_units;
        if projected > event.cap_per_round {
            let remaining = event.cap_per_round - event.issued_in_round;
            token::transfer(ctx.accounts.pool_to_participant(), remaining)?;
            event.issued_in_round = event.cap_per_round;
            return Err(EventErr::Cap.into());
        }

        token::transfer(ctx.accounts.pool_to_participant(), rebate_units)?;
        event.issued_in_round = projected;
        if is_early_bird { event.first_bonus_issued = event.first_bonus_issued + 1; }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEvent<'info> {
    #[account(init, payer = organizer, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub event_state: Account<'info, EventState>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActRebate<'info> {
    #[account(mut, has_one = organizer)]
    pub event_state: Account<'info, EventState>,
    pub organizer: Signer<'info>,

    #[account(mut)]
    pub rebate_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub participant_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActRebate<'info> {
    pub fn pool_to_participant(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let call = Transfer { from: self.rebate_pool_vault.to_account_info(), to: self.participant_vault.to_account_info(), authority: self.organizer.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
}
#[account]
pub struct EventState {
    pub organizer: Pubkey,
    pub base_fee: u64,
    pub cap_per_round: u64,
    pub issued_in_round: u64,
    pub first_bonus_issued: u64,
}
#[error_code]
pub enum EventErr { #[msg("round cap reached")] Cap }
