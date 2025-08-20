use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfADMTOOLS");

#[program]
pub mod nft_admin_tools {
    use super::*;

    /// 指定された NFT アカウントを凍結します。
    ///
    /// ※ `operator` に対する署名者チェックを敢えて省略しているため、
    ///    なりすましによる凍結が可能です。
    pub fn freeze_nft(ctx: Context<FreezeCtx>) -> Result<()> {
        let info = &mut ctx.accounts.nft_info;
        info.frozen = true;
        msg!(
            "VULN: NFT {} frozen by operator {}",
            info.mint,
            ctx.accounts.operator.key()
        );
        Ok(())
    }

    /// 指定された NFT アカウントの凍結を解除します。
    ///
    /// 同様に署名者チェックがありません。
    pub fn unfreeze_nft(ctx: Context<FreezeCtx>) -> Result<()> {
        let info = &mut ctx.accounts.nft_info;
        info.frozen = false;
        msg!(
            "VULN: NFT {} unfrozen by operator {}",
            info.mint,
            ctx.accounts.operator.key()
        );
        Ok(())
    }

    /// ペナルティ率（0–100%）を設定します。
    ///
    /// `admin` に対する検証もなく、誰でも設定可能なままです。
    pub fn set_penalty_rate(ctx: Context<RateCtx>, new_rate: u8) -> Result<()> {
        let cfg = &mut ctx.accounts.penalty_cfg;
        cfg.rate = new_rate;
        msg!(
            "VULN: Penalty rate set to {}% by {}",
            new_rate,
            ctx.accounts.admin.key()
        );
        Ok(())
    }

    /// 凍結中の NFT 保有者からペナルティを徴収し、バイヤルトに蓄えます。
    ///
    /// こちらも `collector` の署名チェックがありません。
    pub fn collect_penalty(ctx: Context<CollectCtx>) -> Result<()> {
        let info   = &mut ctx.accounts.nft_info;
        let vault  = &mut ctx.accounts.penalty_vault;
        let cfg    = &ctx.accounts.penalty_cfg;
        if info.frozen {
            let amount = vault.collected.checked_add(cfg.rate as u64).unwrap();
            vault.collected = amount;
            msg!(
                "VULN: Collected {} tokens from NFT {} (rate {}%)",
                cfg.rate,
                info.mint,
                cfg.rate
            );
        } else {
            msg!("NFT {} is not frozen; no penalty collected", info.mint);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FreezeCtx<'info> {
    #[account(mut, has_one = operator)]
    pub nft_info: Account<'info, NftInfo>,
    /// オペレータ―（**署名チェック省略**）
    pub operator: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RateCtx<'info> {
    #[account(mut, has_one = admin)]
    pub penalty_cfg: Account<'info, PenaltyConfig>,
    /// 管理者（**署名チェック省略**）
    pub admin: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CollectCtx<'info> {
    #[account(mut, has_one = operator)]
    pub nft_info:        Account<'info, NftInfo>,
    #[account(mut, has_one = operator)]
    pub penalty_vault:   Account<'info, PenaltyVault>,
    #[account(has_one = operator)]
    pub penalty_cfg:     Account<'info, PenaltyConfig>,
    /// 徴収者（**署名チェック省略**）
    pub collector:       UncheckedAccount<'info>,
    /// ペナルティ権限を持つ PDA 秘密鍵検証も省略
    pub token_program:   Program<'info, System>,
}

#[account]
pub struct NftInfo {
    pub mint:     Pubkey,  // NFT ミント先
    pub owner:    Pubkey,  // 現在の所有者
    pub operator: Pubkey,  // 凍結／解除権限者
    pub frozen:   bool,    // 凍結中かどうか
}

#[account]
pub struct PenaltyConfig {
    pub admin: Pubkey,
    pub rate:  u8,         // 割合 0～100
}

#[account]
pub struct PenaltyVault {
    pub operator:  Pubkey,
    pub collected: u64,    // 累計ペナルティ
}
