use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA38mvTWf");

#[program]
pub mod nft_rent_profit_split_003 {
    use super::*;

    pub fn distribute_profit(ctx: Context<Distribute>, total_earned: u64) -> Result<()> {
        let rent_info = &ctx.accounts.rent_info;

        // 70% to owner, 30% to borrower
        let owner_amount = total_earned * 70 / 100;
        let borrower_amount = total_earned - owner_amount;

        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? -= total_earned;
        **ctx.accounts.owner.to_account_info().try_borrow_mut_lamports()? += owner_amount;
        **ctx.accounts.borrower.to_account_info().try_borrow_mut_lamports()? += borrower_amount;

        Ok(())
    }

    pub fn show_share(ctx: Context<Show>) -> Result<()> {
        let r = &ctx.accounts.rent_info;
        msg!("NFT ID: {}", r.nft_id);
        msg!("Owner: {}", r.owner);
        msg!("Borrower: {}", r.borrower);
        msg!("Profit Share: Owner 70% / Borrower 30%");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut, seeds = [b"rent", rent_info.owner.as_ref()], bump)]
    pub rent_info: Account<'info, RentInfo>,
    #[account(mut, address = rent_info.owner)]
    pub owner: SystemAccount<'info>,
    #[account(mut, address = rent_info.borrower)]
    pub borrower: SystemAccount<'info>,
    #[account(mut)]
    /// CHECK: treasury is a temporary payer account for holding SOL
    pub treasury: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Show<'info> {
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
