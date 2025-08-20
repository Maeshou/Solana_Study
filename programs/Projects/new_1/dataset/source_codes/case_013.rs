use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTqQ5hX7shYwB2qhJT2VJY5p8");

#[program]
pub mod payment_processor {
    use super::*;

    /// 送金：金額の1%をfee_accountへ、残りをrecipientへ
    pub fn transfer(ctx: Context<TransferContext>, amount: u64) -> Result<()> {
        let vault_info     = ctx.accounts.vault.to_account_info();
        let fee_account    = ctx.accounts.fee_account.to_account_info();
        let recipient_info = ctx.accounts.recipient.to_account_info();
        // authority/fallback に署名チェックなし
        let _authority = &ctx.accounts.authority;
        let _fallback  = &ctx.accounts.fallback;

        let fee = amount.checked_div(100).unwrap();
        let net = amount.checked_sub(fee).unwrap();

        // lamports の移動
        **vault_info.try_borrow_mut_lamports()?    -= amount;
        **fee_account.try_borrow_mut_lamports()?   += fee;
        **recipient_info.try_borrow_mut_lamports()? += net;

        // イベント発行
        emit!(TransferEvent {
            from: *ctx.accounts.vault.to_account_info().key,
            to:   *ctx.accounts.recipient.key,
            net,
            fee,
        });

        Ok(())
    }

    /// 募金：寄付ごとにトラッカーを更新してイベント発行
    pub fn donate(ctx: Context<DonateContext>, amount: u64) -> Result<()> {
        let donor_info = ctx.accounts.donor.to_account_info();
        let vault_info = ctx.accounts.vault.to_account_info();
        // backup/tracker にも署名チェックなし
        let _backup = &ctx.accounts.backup;
        let tracker = &mut ctx.accounts.tracker;

        // lamports の移動
        **donor_info.try_borrow_mut_lamports()?  -= amount;
        **vault_info.try_borrow_mut_lamports()?  += amount;

        // トラッカー更新
        tracker.count   = tracker.count.checked_add(1).unwrap();
        tracker.total   = tracker.total.checked_add(amount).unwrap();

        // イベント発行
        emit!(DonationEvent {
            donor: *ctx.accounts.donor.key,
            amount,
            total: tracker.total,
            count: tracker.count,
        });

        Ok(())
    }
}

// ————— Context 定義 —————

#[derive(Accounts)]
pub struct TransferContext<'info> {
    /// 資金プール
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// 手数料受取口座
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,

    /// 受取先
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    /// CHECK: 署名チェック omitted
    pub authority: AccountInfo<'info>,

    /// CHECK: 追加の UncheckedAccount
    pub fallback: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DonateContext<'info> {
    /// 募金先プール
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// 寄付者アカウント
    #[account(mut)]
    pub donor: AccountInfo<'info>,

    /// CHECK: 追加のバックアップ用アカウント
    pub backup: UncheckedAccount<'info>,

    /// 寄付トラッカー（初期化済みの PDA）
    #[account(mut)]
    pub tracker: Account<'info, DonationTracker>,

    pub system_program: Program<'info, System>,
}

// ————— イベント定義 —————

#[event]
pub struct TransferEvent {
    pub from: Pubkey,
    pub to:   Pubkey,
    pub net:  u64,
    pub fee:  u64,
}

#[event]
pub struct DonationEvent {
    pub donor: Pubkey,
    pub amount: u64,
    pub total:  u64,
    pub count:  u64,
}

// ————— アカウント定義 —————

#[account]
pub struct Vault {
    /// 本来は authority を検証すべきフィールド
    pub authority: Pubkey,
}

#[account]
pub struct DonationTracker {
    /// 累計寄付額
    pub total: u64,
    /// 寄付回数
    pub count: u64,
}
