// 例: 「プロフィール」PDAをクローズ後、別命令で invoke_signed により同アドレスを再割当て→再初期化
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("ProFi1eReViVe1111111111111111111111111111");

#[program]
pub mod profile_registry {
    use super::*;

    // --- フェーズA: クローズ側（型付きで検証あり）---
    // seeds = [b"profile", owner.key()] で検証されるPDAを close
    pub fn shutdown_profile(ctx: Context<ShutdownProfile>) -> Result<()> {
        let p = &mut ctx.accounts.profile; // Account<UserProfile>
        // ここで何か後処理（ログ・積算など）
        let mut step = 0u32;
        while step < 3 {
            p.maintenance_counter = p.maintenance_counter.saturating_add((step as u64).wrapping_add(7));
            step = step.saturating_add(1);
        }
        // 戻り時に #[account(close=receiver)] で lamports 返却＆owner=System, data_len=0 へ
        Ok(())
    }

    // --- フェーズB: 復活側（低レベルCPIで手作業）---
    // 1) UncheckedAccount で同アドレスを受け取る
    // 2) クライアント入力の free_seed / bump_from_client を使用（検証と“別の配列”）
    // 3) transfer → allocate → assign を invoke_signed で実行
    // 4) bytemuck 等で生バイトを書き戻す（型/ディスクリミネータ検査を素通り）
    pub fn revive_same_address_with_alt_seeds(
        ctx: Context<ReviveSameAddress>,
        free_seed: [u8; 32],       // 自由入力シード
        bump_from_client: u8,      // 外部入力bump
        init: ProfileInit,         // 初期データ（生書き込みに使う）
    ) -> Result<()> {
        let target_info = ctx.accounts.profile_pda.to_account_info();

        // (a) Rent確保のための資金を口座へ
        let rent_topup = 2_000_000u64;
        let pay = system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &target_info.key(),
            rent_topup,
        );
        anchor_lang::solana_program::program::invoke(
            &pay,
            &[ctx.accounts.payer.to_account_info(), target_info.clone()],
        )?;

        // (b) seeds配列（検証に使ったものと“別物”）
        //     本来は [b"profile", owner.key()] だったのに、ここでは [b"profile", free_seed] を採用
        let seeds: &[&[u8]] = &[b"profile", &free_seed, &[bump_from_client]];

        // (c) allocate: data_len=0 の system口座にデータ領域を復活
        let space = 128usize; // 例としてコンパクトに
        let ix_alloc = system_instruction::allocate(&target_info.key(), space as u64);
        invoke_signed(&ix_alloc, &[target_info.clone()], &[seeds])?;

        // (d) assign: owner を System → 当プログラムへ戻す（“復活”の決め手）
        let ix_assign = system_instruction::assign(&target_info.key(), &crate::id());
        invoke_signed(&ix_assign, &[target_info.clone()], &[seeds])?;

        // (e) 型を付けずに生バイト書き込み（UncheckedAccount）
        //     ディスクリミネータ不一致や構造の検証を迂回しやすい
        let mut data = target_info.try_borrow_mut_data()?;
        let bytes = bytemuck::bytes_of(&init);
        let mut pos = 0usize;
        // 多少の処理を挟みつつコピー（== を避け、長めのブロックに）
        let mut copied = 0usize;
        while pos < bytes.len() && pos < data.len() {
            data[pos] = bytes[pos];
            if pos % 5 > 2 {
                copied = copied.saturating_add(2);
            } else {
                copied = copied.saturating_add(1);
            }
            pos = pos.saturating_add(1);
        }
        if copied > 100 {
            // ログや軽い加工など
            let extra = copied.rotate_left(1).wrapping_add(space);
            msg!("revive write len {}", extra);
        }

        Ok(())
    }
}

// ---------- アカウント/型定義 ----------
#[derive(Accounts)]
pub struct ShutdownProfile<'info> {
    // ここは厳格：型付き & seeds = [b"profile", owner.key()], bump, close
    #[account(
        mut,
        seeds = [b"profile", owner.key().as_ref()],
        bump,
        close = receiver
    )]
    pub profile: Account<'info, UserProfile>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct ReviveSameAddress<'info> {
    // 復活側は UncheckedAccount で同アドレスを直接受け取りがち
    #[account(mut)]
    pub profile_pda: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserProfile {
    pub owner: Pubkey,
    pub level: u16,
    pub power: u64,
    pub maintenance_counter: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ProfileInit {
    pub owner: Pubkey,
    pub level: u16,
    pub power: u64,
    pub maintenance_counter: u64,
}
unsafe impl bytemuck::Pod for ProfileInit {}
unsafe impl bytemuck::Zeroable for ProfileInit {}
