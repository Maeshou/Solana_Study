use anchor_lang::prelude::*;

declare_id!("Insur4nce111111111111111111111111111111111");

/// 保険契約情報
#[account]
pub struct Policy {
    pub payer:        Pubkey, // 契約者（保険料支払者）
    pub coverage:     u64,    // 補償上限（lamports）
    pub claims_count: u64,    // 請求件数
}

/// 保険金請求情報
#[account]
pub struct Claim {
    pub claimant: Pubkey, // 請求者（必ず payer と同一人物であるべき）
    pub policy:    Pubkey, // 本来はこのフィールドが Policy.key() と一致する必要がある
    pub amount:    u64,    // 請求額
    pub paid:      bool,   // 支払い済みフラグ
}

/// 請求受付イベント
#[event]
pub struct ClaimFiled {
    pub claim:    Pubkey,
    pub policy:   Pubkey,
    pub claimant: Pubkey,
    pub amount:   u64,
}

/// 支払い完了イベント
#[event]
pub struct ClaimPaid {
    pub claim:  Pubkey,
    pub policy: Pubkey,
}

#[derive(Accounts)]
pub struct FileClaim<'info> {
    /// Policy.payer == payer.key() を検証する
    #[account(mut, has_one = payer)]
    pub policy:  Account<'info, Policy>,

    /// 新規 Claim を作成するが、policy フィールドの対応は検証ナシ
    #[account(init, payer = claimant, space = 8 + 32 + 32 + 8 + 1)]
    pub claim:   Account<'info, Claim>,

    #[account(mut)]
    pub claimant: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessClaim<'info> {
    /// Policy.payer == processor.key() は検証されるが…
    #[account(mut, has_one = payer)]
    pub policy:   Account<'info, Policy>,

    /// Claim.policy == policy.key() の検証がないため、
    /// 別のポリシー用に attacker が用意した Claim でも通ってしまう
    #[account(mut)]
    pub claim:    Account<'info, Claim>,

    /// 支払い権限を持つオペレーター
    pub processor: Signer<'info>,
}

#[program]
pub mod insurance_vuln {
    use super::*;

    /// 保険金請求を登録
    pub fn file_claim(ctx: Context<FileClaim>, amount: u64) -> Result<()> {
        let p   = &mut ctx.accounts.policy;
        let c   = &mut ctx.accounts.claim;

        // 脆弱性ポイント：c.policy = p.key() と代入しているだけで、
        // 実際に検証は行っていない
        c.claimant = ctx.accounts.claimant.key();
        c.policy    = p.key();
        c.amount    = amount;
        c.paid      = false;

        p.claims_count = p.claims_count.checked_add(1).unwrap();

        emit!(ClaimFiled {
            claim:    c.key(),
            policy:   p.key(),
            claimant: c.claimant,
            amount,
        });
        Ok(())
    }

    /// 保険金請求を処理
    pub fn process_claim(ctx: Context<ProcessClaim>) -> Result<()> {
        let p = &mut ctx.accounts.policy;
        let c = &mut ctx.accounts.claim;

        // 本来は必須：
        // require_keys_eq!(
        //     c.policy,
        //     p.key(),
        //     InsuranceError::PolicyMismatch
        // );
        //
        // もしくは
        // #[account(address = policy.key())]
        // pub claim: Account<'info, Claim>,
        //
        // がなければ、攻撃者が別の Claim を渡して好き勝手に支払い操作できてしまう。

        // 支払い処理（ダミー）
        if c.amount > p.coverage {
            return err!(InsuranceError::OverCoverage);
        }
        c.paid = true;
        p.coverage = p.coverage.checked_sub(c.amount).unwrap();

        emit!(ClaimPaid {
            claim:  c.key(),
            policy: p.key(),
        });
        Ok(())
    }
}

#[error_code]
pub enum InsuranceError {
    #[msg("Claim が指定の Policy と一致しません")]
    PolicyMismatch,
    #[msg("請求額が補償上限を超えています")]
    OverCoverage,
}
