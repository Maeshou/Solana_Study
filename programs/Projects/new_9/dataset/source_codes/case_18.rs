use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln6666666666666666666666666666666");

#[program]
pub mod marketplace_escrow_revival_demo {
    use super::*;

    pub fn finalize_trade_escrow(ctx: Context<FinalizeTradeEscrow>) -> Result<()> {
        // エスクロー取引を完了して売上を分配
        Ok(())
    }

    pub fn recreate_escrow_same_tx(
        ctx: Context<RecreateEscrowSameTx>,
        escrow_size: u64,
        trade_value: u64,
    ) -> Result<()> {
        let escrow_account = ctx.accounts.trade_escrow_addr.to_account_info();
        let marketplace_operator = ctx.accounts.marketplace_operator.to_account_info();

        let calculated_funding = trade_value / 10;
        let minimum_funding = 800_000;
        let final_amount = std::cmp::max(calculated_funding, minimum_funding);

        let recreate_escrow = system_instruction::transfer(
            &marketplace_operator.key(),
            &escrow_account.key(),
            final_amount
        );
        anchor_lang::solana_program::program::invoke(
            &recreate_escrow,
            &[marketplace_operator.clone(), escrow_account.clone()],
        )?;

        let reserve_escrow_space = system_instruction::allocate(&escrow_account.key(), escrow_size);
        anchor_lang::solana_program::program::invoke(
            &reserve_escrow_space,
            &[escrow_account.clone()]
        )?;

        let control_escrow = system_instruction::assign(&escrow_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(
            &control_escrow,
            &[escrow_account.clone()]
        )?;

        let mut escrow_data = escrow_account.try_borrow_mut_data()?;
        let value_bytes = trade_value.to_le_bytes();
        let bytes_to_write = std::cmp::min(value_bytes.len(), escrow_data.len());
        escrow_data[..bytes_to_write].copy_from_slice(&value_bytes[..bytes_to_write]);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FinalizeTradeEscrow<'info> {
    #[account(mut, close = seller_proceeds)]
    pub trade_escrow: Account<'info, TradeEscrowData>,
    #[account(mut)]
    pub seller_proceeds: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RecreateEscrowSameTx<'info> {
    #[account(mut)]
    pub trade_escrow_addr: UncheckedAccount<'info>,
    #[account(mut)]
    pub marketplace_operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TradeEscrowData {
    pub item_value: u64,
    pub commission_rate: u16,
}