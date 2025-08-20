use anchor_lang::prelude::*;

declare_id!("DupMutAcctMatch3333333333333333333333333333");

#[program]
pub mod update_metadata_and_notify {
    use super::*;

    /// 1) メタデータアカウントを上書きするが、Account Matching を行わず任意のアカウントを読み書きしてしまう  
    /// 2) 同じ型の mutable アカウントを 2 つ受け取るが、キー重複チェックを行わずに両方を操作してしまう  
    pub fn update_metadata_and_notify(
        ctx: Context<UpdateMetadataAndNotify>,
    ) -> Result<()> {
        let metadata1 = &ctx.accounts.metadata_account1;  // 本来は PDA やシードを検証するが省略
        let metadata2 = &ctx.accounts.metadata_account2;  // 同上
        let notify_acc = &ctx.accounts.notify_account;    // 通知用アカウント（Account Matching 省略）
        let authority = &ctx.accounts.authority;

        // --- (1) Account Matching の欠如 ---
        // metadata_account1 / metadata_account2 / notify_account が想定するデータレイアウトや所有者かの検証を行わない。

        // --- (2) Duplicate Mutable Account の欠如 ---
        // metadata1.key() と metadata2.key() が同一の場合もチェックせず、両方に上書きを行う。

        // (1) metadata1, metadata2 の先頭 64 バイトを配列として取得し、カスタムの hash を計算
        let new_hash: [u8; 32] = {
            let raw1 = metadata1.try_borrow_data()?;
            let mut fragment1 = [0u8; 32];
            for i in 0..32 {
                fragment1[i] = raw1[i];
            }
            let raw2 = metadata2.try_borrow_data()?;
            let mut fragment2 = [0u8; 32];
            for i in 0..32 {
                fragment2[i] = raw2[i];
            }
            msg!("[STEP1] fragment1: {:?}, fragment2: {:?}", fragment1, fragment2);
            // 例として、単純に XOR をして新しいハッシュを生成
            let mut computed = [0u8; 32];
            for i in 0..32 {
                computed[i] = fragment1[i] ^ fragment2[i];
            }
            msg!("[STEP1] new_hash を生成: {:?}", computed);
            computed
        };
        // (2) new_hash を metadata1 と metadata2 の先頭 32 バイトに上書き
        {
            let mut raw1_mut = metadata1.try_borrow_mut_data()?;
            for i in 0..32 {
                raw1_mut[i] = new_hash[i];
            }
            msg!("[STEP2] metadata1 に new_hash を上書き");

            let mut raw2_mut = metadata2.try_borrow_mut_data()?;
            for i in 0..32 {
                raw2_mut[i] = new_hash[i];
            }
            msg!("[STEP2] metadata2 に new_hash を上書き");
        }

        // (3) notify_account の「未通知(0) → 通知済み(1)」フラグをセット（複数行扱い）
        {
            let mut raw_notify = notify_acc.try_borrow_mut_data()?;
            if raw_notify[0] == 0 {
                raw_notify[0] = 1;
                msg!("[STEP3] notify_account フラグを 1 にセット");
            } else {
                msg!("[STEP3] notify_account フラグはすでに 1");
            }
        }

        Ok(())
    }
}

/// Context 定義（AccountMatching / PDA 検証を一切行わず、重複チェックも行わない）
#[derive(Accounts)]
pub struct UpdateMetadataAndNotify<'info> {
    /// 本来は PDA の検証を行うべきだが省略
    #[account(mut)] pub metadata_account1: AccountInfo<'info>,
    #[account(mut)] pub metadata_account2: AccountInfo<'info>,

    /// 通知用アカウント（Account Matching 省略）
    #[account(mut)] pub notify_account: AccountInfo<'info>,

    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
