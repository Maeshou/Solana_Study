use anchor_lang::prelude::*;

declare_id!("Hj7p4zGk3cW8jXp5j2b1Q3Y4zGk3cW8jXp5j2b1Q3Y4");

#[program]
pub mod init_pda {
    use super::*;
    pub fn create_user_pda(ctx: Context<CreateUserPda>) -> Result<()> {
        ctx.accounts.user_pda.user = *ctx.accounts.user.key;
        ctx.accounts.user_pda.bump = *ctx.bumps.get("user_pda").unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateUserPda<'info> {
    // PDAを初期化。オーナーは自動でカレントプログラムに設定される
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 1, // Discriminator + Pubkey + u8
        seeds = [b"user-pda", user.key().as_ref()],
        bump
    )]
    pub user_pda: Account<'info, UserPda>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserPda {
    pub user: Pubkey,
    pub bump: u8,
}