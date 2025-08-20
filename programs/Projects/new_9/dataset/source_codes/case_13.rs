use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln1111111111111111111111111111111");

#[program]
pub mod nft_game_revival_demo {
    use super::*;

    pub fn close_player_profile(ctx: Context<ClosePlayerProfile>) -> Result<()> {
        // プレイヤープロファイルを閉じて報酬を収集者に送る
        Ok(())
    }

    pub fn revive_player_same_tx(
        ctx: Context<RevivePlayerSameTx>,
        space: u64,
        player_level: u32,
    ) -> Result<()> {
        let profile_account = ctx.accounts.player_profile_addr.to_account_info();
        let payer_account = ctx.accounts.payer.to_account_info();
        
        let restore_lamports = system_instruction::transfer(
            &payer_account.key(), 
            &profile_account.key(), 
            2_500_000
        );
        anchor_lang::solana_program::program::invoke(
            &restore_lamports,
            &[payer_account.clone(), profile_account.clone()],
        )?;

        let allocate_space = system_instruction::allocate(&profile_account.key(), space);
        anchor_lang::solana_program::program::invoke(
            &allocate_space, 
            &[profile_account.clone()]
        )?;

        let assign_ownership = system_instruction::assign(&profile_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(
            &assign_ownership, 
            &[profile_account.clone()]
        )?;

        let mut account_data = profile_account.try_borrow_mut_data()?;
        for byte_index in 0..4 {
            account_data[byte_index] = ((player_level >> (byte_index * 8)) & 0xff) as u8;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClosePlayerProfile<'info> {
    #[account(mut, close = reward_collector)]
    pub player_profile: Account<'info, PlayerProfileData>,
    #[account(mut)]
    pub reward_collector: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RevivePlayerSameTx<'info> {
    #[account(mut)]
    pub player_profile_addr: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlayerProfileData {
    pub level: u32,
    pub experience_points: u64,
}