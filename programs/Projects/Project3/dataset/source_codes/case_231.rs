use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpMULt1s1gTrAns1GABCDEfghijk");

#[program]
pub mod multisig_transfer {
    use super::*;

    /// ウォレットを初期化: オーナー一覧と閾値を設定
    pub fn init_wallet(
        ctx: Context<InitWallet>,
        owners: Vec<Pubkey>,
        threshold: u8,
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        wallet.owners = owners;
        wallet.threshold = threshold;
        Ok(())
    }

    /// 送金提案の作成（オーナーのみ）
    pub fn propose(
        ctx: Context<Propose>,
        target: Pubkey,
        amount: u64,
    ) -> Result<()> {
        let wallet = &ctx.accounts.wallet;
        // 提案者がオーナーかチェック
        require!(
            wallet.owners.contains(&ctx.accounts.proposer.key()),
            MultisigError::NotOwner
        );

        let prop = &mut ctx.accounts.proposal;
        prop.wallet = wallet.key();
        prop.target = target;
        prop.amount = amount;
        prop.approvals.clear();
        prop.executed = false;
        Ok(())
    }

    /// 提案に対する承認（オーナーのみ）
    pub fn approve(ctx: Context<Approve>) -> Result<()> {
        let wallet = &ctx.accounts.wallet;
        let prop = &mut ctx.accounts.proposal;
        let signer = ctx.accounts.owner.key();

        require!(
            wallet.owners.contains(&signer),
            MultisigError::NotOwner
        );
        if !prop.approvals.contains(&signer) {
            prop.approvals.push(signer);
        }
        Ok(())
    }

    /// 条件を満たせば実行（オーナー署名必須）
    pub fn execute(ctx: Context<Execute>) -> Result<()> {
        let wallet = &ctx.accounts.wallet;
        let prop = &mut ctx.accounts.proposal;
        let authority = ctx.accounts.authority.key();

        // 署名者もオーナーであること
        require!(
            wallet.owners.contains(&authority),
            MultisigError::NotOwner
        );
        // 一度だけ実行
        require!(!prop.executed, MultisigError::AlreadyExecuted);
        // 十分な承認数
        require!(
            prop.approvals.len() as u8 >= wallet.threshold,
            MultisigError::InsufficientApprovals
        );

        // wallet_account が本当に PDA のウォレットかチェック
        let wallet_acc = &mut ctx.accounts.wallet_account.to_account_info();
        require!(
            wallet_acc.key == &wallet.key(),
            MultisigError::MismatchWallet
        );

        // 安全な Lamports transfer
        let target_acc = &mut ctx.accounts.target.to_account_info();
        let from = wallet_acc.lamports();
        let to   = target_acc.lamports();
        let new_from = from.checked_sub(prop.amount).ok_or(MultisigError::InsufficientFunds)?;
        let new_to   = to.checked_add(prop.amount).ok_or(MultisigError::Overflow)?;
        **wallet_acc.try_borrow_mut_lamports()? = new_from;
        **target_acc.try_borrow_mut_lamports()? = new_to;

        prop.executed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWallet<'info> {
    #[account(init, seeds = [b"wallet"], bump, payer = payer, space = 8 + 4 + 32 * 5 + 1)]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)] pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Propose<'info> {
    #[account(mut, seeds = [b"wallet"], bump = wallet.bump)]
    pub wallet: Account<'info, Wallet>,
    #[account(init, seeds = [b"proposal", wallet.key().as_ref()], bump, payer = proposer, space = 8 + Proposal::SIZE)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)] pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut, seeds = [b"proposal", proposal.wallet.as_ref()], bump = proposal.bump, has_one = wallet @ MultisigError::MismatchWallet)]
    pub proposal: Account<'info, Proposal>,
    pub wallet: Account<'info, Wallet>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Execute<'info> {
    /// 実際の Lamports を保持するのはこの PDA ウォレット
    #[account(mut)]
    pub wallet_account: AccountInfo<'info>,
    #[account(mut)]
    pub target: AccountInfo<'info>,
    pub wallet: Account<'info, Wallet>,
    #[account(mut, seeds = [b"proposal", proposal.wallet.as_ref()], bump = proposal.bump, has_one = wallet @ MultisigError::MismatchWallet)]
    pub proposal: Account<'info, Proposal>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Wallet {
    pub owners:    Vec<Pubkey>,
    pub threshold: u8,
    pub bump:      u8,
}

#[account]
pub struct Proposal {
    pub wallet:    Pubkey,
    pub target:    Pubkey,
    pub amount:    u64,
    pub approvals: Vec<Pubkey>,
    pub executed:  bool,
    pub bump:      u8,
}

impl Proposal {
    // Vec<Pubkey> の長さ分を加味したサイズ計算
    pub const SIZE: usize = 32 + 32 + 8 + 4 + 32 * 5 + 1 + 1;
}

#[error]
pub enum MultisigError {
    #[msg("Caller is not an owner.")]
    NotOwner,
    #[msg("Proposal does not belong to this wallet.")]
    MismatchWallet,
    #[msg("Already executed.")]
    AlreadyExecuted,
    #[msg("Insufficient approvals.")]
    InsufficientApprovals,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
    #[msg("Arithmetic overflow.")]
    Overflow,
}
