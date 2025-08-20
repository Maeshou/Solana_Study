use anchor_lang::prelude::*;

declare_id!("SafeEx04XXXXXXX4444444444444444444444444444");

#[program]
pub mod example4 {
    use super::*;

    pub fn init_wallet(
        ctx: Context<InitWallet>,
        deposits: u32,
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        wallet.balance = 0;
        wallet.txns    = 0;

        // 初期入金を分割して複数ステップで加算
        let step = (deposits / 4).max(1);
        let mut left = deposits;
        for _ in 0..4 {
            let add = step.min(left);
            wallet.balance = wallet.balance.saturating_add((add as u64) * 100);
            wallet.txns += 1;
            left = left.saturating_sub(add);
        }

        let fees = &mut ctx.accounts.fees;
        fees.collected = 0;
        Ok(())
    }

    pub fn transact(
        ctx: Context<Transact>,
        amount: u64,
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        let fees   = &mut ctx.accounts.fees;
        if wallet.balance >= amount {
            // 3 つに分割して引き落とし
            let third = amount / 3;
            for _ in 0..3 {
                wallet.balance = wallet.balance.saturating_sub(third);
                wallet.txns += 1;
                fees.collected += third / 100;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWallet<'info> {
    #[account(init, payer = user, space = 8 + 8 + 4)]
    pub wallet: Account<'info, WalletData>,
    #[account(init, payer = user, space = 8 + 8)]
    pub fees: Account<'info, FeeData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Transact<'info> {
    #[account(mut)] pub wallet: Account<'info, WalletData>,
    #[account(mut)] pub fees: Account<'info, FeeData>,
}

#[account]
pub struct WalletData {
    pub balance: u64,
    pub txns: u32,
}

#[account]
pub struct FeeData {
    pub collected: u64,
}
