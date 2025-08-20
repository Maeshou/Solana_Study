use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_staking_game {
    use super::*;

    pub fn stake_nft(ctx: Context<StakeNft>, nft_mint_address: Pubkey) -> Result<()> {
        let stake_record = &mut ctx.accounts.stake_record;
        let clock = Clock::get()?;
        
        stake_record.owner = *ctx.accounts.staker.key;
        stake_record.nft_mint = nft_mint_address;
        stake_record.start_time = clock.unix_timestamp;
        stake_record.is_active = true;
        stake_record.bump = *ctx.bumps.get("stake_record").unwrap();
        
        // ダミーのwhileループ
        let mut validation_count = 0;
        while validation_count < 3 {
             msg!("Validating NFT ownership... step {}", validation_count + 1);
             // ここで実際にNFTの所有権を検証する処理が入る
             validation_count += 1;
        }

        msg!("NFT {} has been staked by {}.", stake_record.nft_mint, stake_record.owner);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(nft_mint_address: Pubkey)]
pub struct StakeNft<'info> {
    #[account(
        init,
        payer = staker,
        space = 8 + 32 + 32 + 8 + 1 + 1,
        seeds = [b"stake", staker.key().as_ref(), nft_mint_address.as_ref()],
        bump
    )]
    pub stake_record: Account<'info, StakeRecord>,
    #[account(mut)]
    pub staker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeRecord {
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub start_time: i64,
    pub is_active: bool,
    pub bump: u8,
}