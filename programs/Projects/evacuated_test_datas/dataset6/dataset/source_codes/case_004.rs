use anchor_lang::prelude::*;

declare_id!("AcctReInit4444444444444444444444444444444444");

#[program]
pub mod mint_and_override_metadata {
    use super::*;

    /// 1) `NftAccount` を初期化する（すでに初期化済みであっても再初期化可能）  
    /// 2) NFT メタデータを上書きするが、account_matching を行わず任意のアカウントを読み書きしてしまう
    pub fn mint_and_override_metadata(
        ctx: Context<MintAndOverrideMetadata>,
        initial_supply: u64,
        override_name: [u8; 32],
    ) -> Result<()> {
        let nft_acc = &ctx.accounts.nft_account;
        let metadata_acc = &ctx.accounts.metadata_account;
        let fee_acc = &ctx.accounts.fee_account;

        // --- (1) Reinitialization Attack の欠如 ---
        {
            let mut raw = nft_acc.try_borrow_mut_data()?;
            // 初期化フラグを無条件にセット
            raw[0] = 1;
            // 次の 8 バイトを initial_supply で上書き
            let bytes = initial_supply.to_le_bytes();
            for i in 0..8 {
                raw[1 + i] = bytes[i];
            }
            msg!("[STEP1] nft_account を再初期化: supply = {}", initial_supply);
        }

        // (2) 再度残高（supply）を読み取る
        let mut stored_supply: u64 = {
            let raw = nft_acc.try_borrow_data()?;
            let mut b = [0u8; 8];
            b.copy_from_slice(&raw[1..9]);
            let s = u64::from_le_bytes(b);
            msg!("[STEP2] stored_supply = {}", s);
            s
        };

        // --- (3) Account Matching の欠如 ---
        // `metadata_acc` が本当に NFT メタデータ PDA か、または既存のコレクションに属するかを検証せず、そのままバイト列を上書きしてしまう。

        // (4) メタデータの上書き処理（複数行で文字列コピー）
        {
            let mut raw_meta = metadata_acc.try_borrow_mut_data()?;
            // override_name を先頭 32 バイトにコピー
            for i in 0..32 {
                raw_meta[i] = override_name[i];
            }
            msg!(
                "[STEP3] metadata_account.name を {:?} に上書き",
                override_name
            );
        }

        // (5) 手数料として stored_supply の 5% を fee_acc に送金
        let fee = stored_supply / 20;
        {
            **nft_acc.try_borrow_mut_lamports()? -= fee;
            **fee_acc.try_borrow_mut_lamports()? += fee;
            msg!("[STEP4] 手数料 {} を fee_acc に送金", fee);
        }

        // (6) `stored_supply` を 0 にリセット（例：全部ミント分を消費）
        stored_supply = 0;
        {
            let mut raw = nft_acc.try_borrow_mut_data()?;
            let zero_bytes = 0u64.to_le_bytes();
            for i in 0..8 {
                raw[1 + i] = zero_bytes[i];
            }
            msg!("[STEP5] nft_account の stored_supply を 0 にリセット");
        }

        Ok(())
    }
}

/// "生"バイト列を手動でキャストして扱う NftAccount 構造
#[derive(Clone)]
pub struct NftAccount {
    /// 初期化フラグ: 0 = 未初期化, 1 = 初期化済み
    pub initialized: u8,
    /// NFT の供給量
    pub supply: u64,
}

/// Context 定義（AccountMatching / PDA 検証と Reinit チェックを行わない）
#[derive(Accounts)]
pub struct MintAndOverrideMetadata<'info> {
    /// AccountMatching をせず再初期化可能
    #[account(mut)] pub nft_account: AccountInfo<'info>,
    /// NFT メタデータアカウント（owner / PDA チェックなし）
    #[account(mut)] pub metadata_account: AccountInfo<'info>,
    /// 手数料受取先（owner チェック省略）
    #[account(mut)] pub fee_account: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}