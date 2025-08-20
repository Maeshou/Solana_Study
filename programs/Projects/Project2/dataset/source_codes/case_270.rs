use anchor_lang::prelude::*;

declare_id!("Bracket0999999999999999999999999999999999");

#[program]
pub mod elimination_bracket {
    use super::*;

    pub fn register(ctx: Context<ModifyBracket>, slot: u8) -> Result<()> {
        let eb = &mut ctx.accounts.bracket;
        if (slot as usize) < eb.slots.len() {
            eb.slots[slot as usize] = Some(ctx.accounts.user.key());
            eb.registered = eb.registered.saturating_add(1);
        }
        Ok(())
    }

    pub fn clear_slot(ctx: Context<ModifyBracket>, slot: u8) -> Result<()> {
        let eb = &mut ctx.accounts.bracket;
        if (slot as usize) < eb.slots.len() && eb.slots[slot as usize].is_some() {
            eb.slots[slot as usize] = None;
            eb.registered = eb.registered.saturating_sub(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyBracket<'info> {
    #[account(mut)]
    pub bracket: Account<'info, BracketData>,
    pub user: Signer<'info>,
}

#[account]
pub struct BracketData {
    pub slots: [Option<Pubkey>; 16],
    pub registered: u64,
}
