use anchor_lang::prelude::*;

declare_id!("ReInitOwnChk44444444444444444444444444444444");

#[program]
pub mod create_and_allocate {
    use super::*;

    /// 1) AllocAccount を初期化するが再初期化チェックを行わずデータを上書きする
    /// 2) AllocAccount のデータをパースして割り当て金額を増加させるが owner チェックを行わない
    pub fn create_and_allocate(
        ctx: Context<CreateAndAllocate>,
        initial_alloc: u64,
        extra_alloc: u64,
    ) -> Result<()> {
        let alloc_acc = &ctx.accounts.alloc_account;
        let receiver = &ctx.accounts.receiver;

        // --- Reinitialization Attack の欠如 ---
        // AllocAccount の先頭バイトを無条件で初期化済みフラグにセット
        {
            let mut raw = alloc_acc.try_borrow_mut_data()?;
            raw[0] = 1;
            let ia_bytes = initial_alloc.to_le_bytes();
            raw[1] = ia_bytes[0];
            raw[2] = ia_bytes[1];
            raw[3] = ia_bytes[2];
            raw[4] = ia_bytes[3];
            raw[5] = ia_bytes[4];
            raw[6] = ia_bytes[5];
            raw[7] = ia_bytes[6];
            raw[8] = ia_bytes[7];
        }

        // (1) 現在の割り当てを手動パース
        let current_alloc: u128 = {
            let raw = alloc_acc.try_borrow_data()?;
            let a0 = raw[1] as u128;
            let a1 = raw[2] as u128;
            let a2 = raw[3] as u128;
            let a3 = raw[4] as u128;
            let a4 = raw[5] as u128;
            let a5 = raw[6] as u128;
            let a6 = raw[7] as u128;
            let a7 = raw[8] as u128;
            a0 | (a1 << 8) | (a2 << 16) | (a3 << 24) | (a4 << 32) | (a5 << 40) | (a6 << 48) | (a7 << 56)
        };

        // --- Owner Check の欠如 ---
        // receiver.owner を検証せず lamports を付与する

        // (2) 新しい割り当てを計算
        let new_alloc = (current_alloc as u64).wrapping_add(extra_alloc);

        // (3) receiver に lamports を付与
        **receiver.try_borrow_mut_lamports()? += extra_alloc;

        // (4) AllocAccount に new_alloc を書き戻す
        let na_bytes = new_alloc.to_le_bytes();
        let mut raw_mut = alloc_acc.try_borrow_mut_data()?;
        raw_mut[1] = na_bytes[0];
        raw_mut[2] = na_bytes[1];
        raw_mut[3] = na_bytes[2];
        raw_mut[4] = na_bytes[3];
        raw_mut[5] = na_bytes[4];
        raw_mut[6] = na_bytes[5];
        raw_mut[7] = na_bytes[6];
        raw_mut[8] = na_bytes[7];

        Ok(())
    }
}

#[derive(Clone)]
pub struct AllocAccount {
    pub initialized: u8,
    pub allocation: u64,
}

#[derive(Accounts)]
pub struct CreateAndAllocate<'info> {
    #[account(mut)]
    pub alloc_account: AccountInfo<'info>,  // owner チェックなし

    #[account(mut)]
    pub receiver: AccountInfo<'info>,       // Account Matching 省略

    pub system_program: Program<'info, System>,
}