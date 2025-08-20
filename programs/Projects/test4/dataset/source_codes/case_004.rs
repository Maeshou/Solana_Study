#[program]
pub mod insecure_transfer {
    use super::*;

    pub fn transfer_points(ctx: Context<TransferPoints>, amount: u64) -> Result<()> {
        let sender = &mut ctx.accounts.sender;
        let receiver = &mut ctx.accounts.receiver;

        sender.points -= amount;
        receiver.points += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferPoints<'info> {
    #[account(mut)]
    pub sender: Account<'info, User>,
    #[account(mut)]
    pub receiver: Account<'info, User>
    // Signer 型がないため、署名チェックが一切行われない
}

#[account]
pub struct User {
    pub points: u64,
}