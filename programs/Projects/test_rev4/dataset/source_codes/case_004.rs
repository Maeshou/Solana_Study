use anchor_lang::prelude::*;

declare_id!("Trans444444444444444444444444444444444444444");

#[program]
pub mod insecure_transfer {
    use super::*;

    pub fn transfer_points(ctx: Context<TransferPoints>, amount: u64) -> Result<()> {
        let sender_info = ctx.accounts.sender.to_account_info();
        let receiver_info = ctx.accounts.receiver.to_account_info();

        // lamports の増減処理
        **sender_info.try_borrow_mut_lamports()? = sender_info
            .lamports()
            .checked_sub(amount)
            .unwrap();
        **receiver_info.try_borrow_mut_lamports()? = receiver_info
            .lamports()
            .checked_add(amount)
            .unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferPoints<'info> {
    #[account(mut)]
    pub sender: AccountInfo<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    /// ここで署名者チェックを追加
    pub signer: Signer<'info>,
}