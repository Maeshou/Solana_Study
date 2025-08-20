use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln3333333333333333333333333333333");

#[program]
pub mod guild_token_revival_demo {
    use super::*;

    pub fn dissolve_guild_treasury(ctx: Context<DissolveGuildTreasury>) -> Result<()> {
        // ギルドの財宝庫を解散して資金を分配
        Ok(())
    }

    pub fn rebuild_treasury_same_tx(
        ctx: Context<RebuildTreasurySameTx>,
        data_capacity: u64,
        initial_gold: u64,
    ) -> Result<()> {
        let treasury_account = ctx.accounts.guild_treasury_addr.to_account_info();
        let guild_master = ctx.accounts.guild_master.to_account_info();

        let restoration_amount = initial_gold * 100_000;
        let refund_treasury = system_instruction::transfer(
            &guild_master.key(),
            &treasury_account.key(),
            restoration_amount
        );
        anchor_lang::solana_program::program::invoke(
            &refund_treasury,
            &[guild_master.clone(), treasury_account.clone()],
        )?;

        let expand_storage = system_instruction::allocate(&treasury_account.key(), data_capacity);
        anchor_lang::solana_program::program::invoke(
            &expand_storage,
            &[treasury_account.clone()]
        )?;

        let claim_ownership = system_instruction::assign(&treasury_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(
            &claim_ownership,
            &[treasury_account.clone()]
        )?;

        let mut treasury_data = treasury_account.try_borrow_mut_data()?;
        let gold_bytes = initial_gold.to_le_bytes();
        for (position, gold_byte) in gold_bytes.iter().enumerate() {
            treasury_data[position] = *gold_byte;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DissolveGuildTreasury<'info> {
    #[account(mut, close = distribution_fund)]
    pub guild_treasury: Account<'info, GuildTreasuryData>,
    #[account(mut)]
    pub distribution_fund: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RebuildTreasurySameTx<'info> {
    #[account(mut)]
    pub guild_treasury_addr: UncheckedAccount<'info>,
    #[account(mut)]
    pub guild_master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GuildTreasuryData {
    pub gold_amount: u64,
    pub member_count: u32,
}