use anchor_lang::prelude::*;

declare_id!("2b1Q3Y4zGk3cW8jXp5j2b1Q3Y4zGk3cW8jXp5jHj7p4");

#[program]
pub mod close_account_example {
    use super::*;
    pub fn close_account(_ctx: Context<CloseAccount>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    // my_accountのオーナーがカレントプログラムであり、
    // authorityが権限者であることを検証してから閉じる
    #[account(mut, has_one = authority, close = receiver)]
    pub my_account: Account<'info, MyData>,
    #[account(mut)]
    pub receiver: Signer<'info>, // レントの受取人
    pub authority: Signer<'info>,
}

#[account]
pub struct MyData {
    pub data: u64,
    pub authority: Pubkey,
}