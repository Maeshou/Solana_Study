use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod nft_reward_simple {
    use super::*;

    /// 設定：NFT 1枚あたりの報酬 lamports を設定
    pub fn configure(ctx: Context<Configure>, lamports_per_nft: u64) -> ProgramResult {
        let cfg = &mut ctx.accounts.config;
        cfg.lamports_per_nft = lamports_per_nft;
        Ok(())
    }

    /// 引き出し：NFT 枚数に応じて vault から recipient へ lamports を直接移動
    /// CPI も Result<T,_> も使わず、ProgramResult のみ
    pub fn withdraw(ctx: Context<Withdraw>, nft_count: u64) -> ProgramResult {
        let cfg = &ctx.accounts.config;
        let vault = ctx.accounts.vault.to_account_info();
        let recipient = ctx.accounts.recipient.to_account_info();

        // オーバーフローチェック
        require!(
            cfg.lamports_per_nft.checked_mul(nft_count).is_some(),
            ErrorCode::Overflow
        );
        let total = cfg.lamports_per_nft.checked_mul(nft_count).unwrap();

        // 残高チェック
        let vault_balance = **vault.lamports.borrow();
        require!(vault_balance >= total, ErrorCode::InsufficientFunds);

        // lamports を直接移動
        **vault.lamports.borrow_mut() = vault_balance - total;
        **recipient.lamports.borrow_mut() += total;

        Ok(())
    }
}

#[account]
pub struct Config {
    /// NFT 1枚あたりの lamports 報酬
    pub lamports_per_nft: u64,
}

#[derive(Accounts)]
pub struct Configure<'info> {
    #[account(init, payer = payer, space = 8 + 8)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// 報酬保管用アカウント
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    /// 報酬受取先アカウント
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub config: Account<'info, Config>,
}

