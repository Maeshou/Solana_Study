use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA37mvTWf");

#[program]
pub mod nft_rent_confirm_003 {
    use super::*;

    pub fn register_rental(ctx: Context<RegisterCtx>, nft_id: u64, start_slot: u64) -> Result<()> {
        let rent = &mut ctx.accounts.rent_info;
        rent.owner = ctx.accounts.owner.key();
        rent.nft_id = nft_id;
        rent.start_slot = start_slot;
        rent.borrower = Pubkey::default(); // まだ未指定
        Ok(())
    }

    pub fn confirm_borrower(ctx: Context<ConfirmCtx>) -> Result<()> {
        let rent = &mut ctx.accounts.rent_info;
        rent.borrower = ctx.accounts.borrower.key(); // 借主の記録（誰が借りるか）
        Ok(())
    }

    pub fn show(ctx: Context<ShowCtx>) -> Result<()> {
        let r = &ctx.accounts.rent_info;
        msg!("NFT ID: {}", r.nft_id);
        msg!("Start Slot: {}", r.start_slot);
        msg!("Owner: {}", r.owner);
        msg!("Borrower: {}", r.borrower);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct RegisterCtx<'info> {
    #[account(init_if_needed, payer = owner, space = 8 + 32 + 32 + 8 + 8, seeds = [b"rent", owner.key().as_ref()], bump)]
    pub rent_info: Account<'info, RentInfo>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmCtx<'info> {
    #[account(mut, seeds = [b"rent", rent_info.owner.as_ref()], bump)]
    pub rent_info: Account<'info, RentInfo>,
    pub borrower: Signer<'info>,
}

#[derive(Accounts)]
pub struct ShowCtx<'info> {
    #[account(seeds = [b"rent", rent_info.owner.as_ref()], bump)]
    pub rent_info: Account<'info, RentInfo>,
}

#[account]
pub struct RentInfo {
    pub owner: Pubkey,
    pub borrower: Pubkey,
    pub nft_id: u64,
    pub start_slot: u64,
}
