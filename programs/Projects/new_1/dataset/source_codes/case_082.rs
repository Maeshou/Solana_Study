use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpAbCdEfGhIjKlMnOpQrStUvWxY99");

#[program]
pub mod nft_reward_no_cpi {
    use super::*;

    /// NFT あたりの報酬 lamports を設定する
    pub fn configure(
        ctx: Context<Configure>,
        lamports_per_nft: u64,
    ) -> ProgramResult {
        let cfg = &mut ctx.accounts.config;
        cfg.lamports_per_nft = lamports_per_nft;
        Ok(())
    }

    /// NFT 数に応じた lamports を引き出し、vault から recipient へ直接移動させる
    /// CPI は一切使わず、直接 lamports 値を操作する
    pub fn withdraw(
        ctx: Context<Withdraw>,
        nft_count: u64,
    ) -> Result<u64> {
        let cfg = &ctx.accounts.config;
        let vault_info = ctx.accounts.vault.to_account_info();
        let recipient_info = ctx.accounts.recipient.to_account_info();

        // 計算
        let total = cfg
            .lamports_per_nft
            .checked_mul(nft_count)
            .ok_or(ErrorCode::Overflow)?;

        // 残高チェック
        let vault_balance = **vault_info.lamports.borrow();
        if vault_balance < total {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // lamports を直接移動
        **vault_info.lamports.borrow_mut() = vault_balance - total;
        **recipient_info.lamports.borrow_mut() = 
            **recipient_info.lamports.borrow() + total;

        // 引き出した量を返す
        Ok(total)
    }
}

#[account]
pub struct Config {
    /// NFT 1枚あたりの lamports 報酬
    pub lamports_per_nft: u64,
}

#[derive(Accounts)]
pub struct Configure<'info> {
    /// 設定アカウント（PDA 等でも可）
    #[account(init, payer = payer, space = 8 + 8)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// 報酬支払い用 vault（lamports を保有する任意のアカウント）
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    /// 報酬受取先アカウント
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    /// 設定アカウント
    pub config: Account<'info, Config>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("計算時にオーバーフローしました")]
    Overflow,
    #[msg("vault の残高が不足しています")]
    InsufficientFunds,
}
