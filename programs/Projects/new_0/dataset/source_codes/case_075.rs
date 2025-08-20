use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfAirdrop00");

#[program]
pub mod simple_airdrop {
    use super::*;

    /// 初回登録時にのみアカウントを初期化し、配布量を設定します。
    pub fn register(ctx: Context<Register>, total_amount: u64) -> Result<()> {
        let rec = &mut ctx.accounts.airdrop_record;
        rec.user = ctx.accounts.user.key();
        rec.registered = 1;
        rec.total_amount = total_amount;
        rec.claimed_amount = 0;
        Ok(())
    }

    /// 登録済みユーザーは、設定された総配布量を一度だけ請求できます。
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let rec = &mut ctx.accounts.airdrop_record;
        let can = rec.registered.saturating_sub(1);          // 登録フラグチェック
        let to_pay = rec.total_amount.saturating_mul(can as u64);
        let pay = to_pay.saturating_sub(rec.claimed_amount);  // 未請求分だけ
        rec.claimed_amount = rec.claimed_amount.saturating_add(pay);
        msg!("Airdropped {} tokens to {:?}", pay, rec.user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Register<'info> {
    /// 初回のみ PDA を生成して初期化
    #[account(
        init,
        payer = user,
        space  = 8 + 32 + 1 + 8 + 8,
        seeds = [b"airdrop", user.key().as_ref()],
        bump
    )]
    pub airdrop_record: Account<'info, AirdropRecord>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    /// PDA と所有者チェックで不正アクセス防止
    #[account(
        seeds = [b"airdrop", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub airdrop_record: Account<'info, AirdropRecord>,

    pub user: Signer<'info>,
}

#[account]
pub struct AirdropRecord {
    pub user: Pubkey,         // レコード所有者
    pub registered: u8,       // 登録済みフラグ (0 or 1)
    pub total_amount: u64,    // 総配布予定量
    pub claimed_amount: u64,  // これまでに請求した量
}
