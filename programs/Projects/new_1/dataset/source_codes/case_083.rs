use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpNEWPrgrmIDforNFTdistr01");

#[program]
pub mod nft_reward_simple_no_errors {
    use super::*;

    /// 1. 報酬レート設定：NFT 1枚あたりの lamports を登録する
    pub fn set_reward_rate(
        ctx: Context<SetRate>,
        lamports_per_nft: u64,
    ) -> ProgramResult {
        let cfg = &mut ctx.accounts.rate_config;
        cfg.lamports_per_nft = lamports_per_nft;
        Ok(())
    }

    /// 2. 報酬配布：指定枚数分の lamports を vault から受取アカウントへ移す  
    ///    オーバーフローや残高不足はすべて `unwrap()` でパニック扱い
    pub fn distribute_rewards(
        ctx: Context<Distribute>,
        nft_count: u64,
    ) -> ProgramResult {
        let cfg = &ctx.accounts.rate_config;
        let vault = ctx.accounts.vault.to_account_info();
        let recipient = ctx.accounts.recipient.to_account_info();

        // 乗算オーバーフロー時はパニック
        let total = cfg.lamports_per_nft.checked_mul(nft_count).unwrap();

        // 直接 lamports を移動（残高不足もパニック）
        **vault.lamports.borrow_mut() -= total;
        **recipient.lamports.borrow_mut() += total;

        Ok(())
    }
}

/// lamports_per_nft を保持するだけのシンプル構造体
#[account]
pub struct RateConfig {
    pub lamports_per_nft: u64,
}

#[derive(Accounts)]
pub struct SetRate<'info> {
    #[account(init, payer = authority, space = 8 + 8)]
    pub rate_config: Account<'info, RateConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    /// 報酬保管用アカウント
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    /// 報酬受取先アカウント
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    /// レート設定アカウント
    pub rate_config: Account<'info, RateConfig>,
}
