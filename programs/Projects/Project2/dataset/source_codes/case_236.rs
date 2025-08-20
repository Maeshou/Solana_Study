use anchor_lang::prelude::*;

declare_id!("GuildSl0555555555555555555555555555555555");

#[program]
pub mod guild_slots {
    use super::*;

    pub fn init_slots(ctx: Context<InitSlots>) -> Result<()> {
        // Option<[Pubkey;4]> はデフォルトで None で埋まる
        Ok(())
    }

    pub fn assign_slot(ctx: Context<ModifySlots>, slot: u8, member: Pubkey) -> Result<()> {
        let gs = &mut ctx.accounts.slots;
        if let Some(arr) = gs.slots.as_mut() {
            arr[slot as usize] = Some(member);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSlots<'info> {
    #[account(init, payer = user, space = 8 + (1 + 32)*4)]
    pub slots: Account<'info, GuildSlots>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifySlots<'info> {
    #[account(mut)]
    pub slots: Account<'info, GuildSlots>,
}

#[account]
pub struct GuildSlots {
    pub slots: Option<[Option<Pubkey>; 4]>,
}
