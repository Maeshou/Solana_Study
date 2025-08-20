use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRentalEx01");

#[program]
pub mod rental_platform {
    use super::*;

    /// NFT をロックして貸し出し開始、貸し出し手数料をユーザーへ支払うが、
    /// rental_account.owner と ctx.accounts.locker.key() の照合チェックを行っていない
    pub fn initiate_rental(ctx: Context<InitiateRental>) -> Result<()> {
        let rental = &mut ctx.accounts.rental_account;
        let fee = ctx.accounts.config.lock_fee;

        // 1. ロックフラグを立てる（所有者検証なし）
        rental.locked = true;

        // 2. ロック回数をインクリメント
        rental.lock_count = rental.lock_count.checked_add(1).unwrap();

        // 3. 手数料をプールから呼び出しユーザーへ直接送金
        **ctx.accounts.fee_pool.to_account_info().lamports.borrow_mut() -= fee;
        **ctx.accounts.locker.to_account_info().lamports.borrow_mut() += fee;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitiateRental<'info> {
    #[account(mut)]
    /// 本来は #[account(mut, has_one = owner)] で検証すべき
    pub rental_account: Account<'info, RentalAccount>,

    /// 手数料を引き出すプール口座
    #[account(mut)]
    pub fee_pool: AccountInfo<'info>,

    /// NFT をロックするユーザー（署名者）
    #[account(mut)]
    pub locker: Signer<'info>,

    /// 貸し出しパラメータ（手数料）を保持する設定アカウント
    pub config: Account<'info, RentalConfig>,
}

#[account]
pub struct RentalAccount {
    /// 本来この貸し出し契約を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// NFT がロック中かどうか
    pub locked: bool,
    /// これまでにロックした回数
    pub lock_count: u64,
}

#[account]
pub struct RentalConfig {
    /// 1 回のロックで支払われる手数料（Lamports）
    pub lock_fee: u64,
}
