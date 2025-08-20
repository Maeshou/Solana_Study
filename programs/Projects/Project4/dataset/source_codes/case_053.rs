use anchor_lang::prelude::*;

declare_id!("SafeEx04XXXXXXX4444444444444444444444444444");

#[program]
pub mod example4 {
    use super::*;

    pub fn init_wallet(
        ctx: Context<InitWallet>,
        initial_deposits: u32,
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        wallet.balance = 0;
        wallet.txns = 0;
        for _ in 0..initial_deposits {
            wallet.balance = wallet.balance.saturating_add(1_000);
            wallet.txns += 1;
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
        if wallet.balance >= amount {
            wallet.balance -= amount;
            wallet.txns += 1;
            let fees = &mut ctx.accounts.fees;
            // 手数料は取引額の1%
            fees.collected += amount / 100;
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
