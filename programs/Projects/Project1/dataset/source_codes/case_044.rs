use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA34mvTWf");

#[program]
pub mod resource_accumulator_003 {
    use super::*;

    pub fn gather(ctx: Context<Ctx003>, resource_type: u64, amount: u64) -> Result<()> {
        let store = &mut ctx.accounts.player_storage;

        let r0 = (resource_type == 0) as u64;
        let r1 = (resource_type == 1) as u64;
        let r2 = (resource_type == 2) as u64;

        store.wood += r0 * amount;
        store.stone += r1 * amount;
        store.metal += r2 * amount;

        Ok(())
    }

    pub fn show(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.player_storage;
        msg!("Wood: {}", s.wood);
        msg!("Stone: {}", s.stone);
        msg!("Metal: {}", s.metal);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = player)]
    pub player_storage: Account<'info, PlayerStorage>,
    #[account(signer)]
    pub player: Signer<'info>,
}

#[account]
pub struct PlayerStorage {
    pub player: Pubkey,
    pub wood: u64,
    pub stone: u64,
    pub metal: u64,
}
