use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("ArTiFaCtW0rKsHoP111111111111111111111111");

#[program]
pub mod artifact_workshop {
    use super::*;

    /// メインのPDAは Anchor の #[account(seeds, bump)] で検証・安全
    pub fn setup_workshop(ctx: Context<SetupWorkshop>, energy: u64) -> Result<()> {
        let ws = &mut ctx.accounts.workshop;
        ws.owner = ctx.accounts.owner.key();
        ws.workshop_bump = *ctx.bumps.get("workshop").ok_or(error!(Errs::MissingBump))?;
        ws.energy = energy % 700 + 100;
        ws.quality = 1;

        // いくつかの初期演算：while→if を交互に
        let mut t = 0u8;
        while t < 6 {
            ws.energy = ws.energy.rotate_left(1).wrapping_add((t as u64) * 17 + 9);
            if (ws.energy & 1) == 1 {
                ws.quality = ws.quality.saturating_add((ws.energy % 23) as u32 + 7);
                ws.energy = ws.energy.wrapping_mul(2).wrapping_add(13);
            } else {
                ws.quality = ws.quality.saturating_add((ws.energy % 19) as u32 + 8);
                ws.energy = ws.energy.rotate_right(2).wrapping_add(11);
            }
            t = t.saturating_add(1);
        }

        if ws.quality % 2 == 0 {
            ws.energy = ws.energy.wrapping_mul(3).wrapping_add(29);
            ws.quality = ws.quality.saturating_add(5);
        } else {
            ws.energy = ws.energy.wrapping_mul(2).wrapping_add(31);
            ws.quality = ws.quality.saturating_add(9);
        }

        Ok(())
    }

    /// 問題のコア：
    /// - workshop は #[account(seeds=[b"workshop", owner], bump)] で安全に検証済み
    /// - しかし「別の関連PDA bench」を *手動* で導出し、ユーザ由来の user_bump で invoke_signed してしまう
    /// - bench アカウント自体は #[account(seeds=...)] で検証していない
    /// → Bump Seed Canonicalization が bench 側で再発
    pub fn craft_and_withdraw(
        ctx: Context<CraftAndWithdraw>,
        item_tag: String,
        user_bump: u8,         // ← ここが危険。保存済み or 入力の bump をそのまま使ってしまう
        amount: u64,
    ) -> Result<()> {
        let ws = &mut ctx.accounts.workshop;

        // 内部の加工ロジック（for→if の順番入れ替えなどで単調にしない）
        for step in 0..7 {
            let addq = ((ws.energy % 37) as u32).saturating_add(step as u32 + 6);
            ws.quality = ws.quality.saturating_add(addq);
            ws.energy = ws.energy.wrapping_add((step as u64) * 21 + 5);
        }

        if ws.energy & 4 == 4 {
            ws.quality = ws.quality.saturating_add(13);
            ws.energy = ws.energy.rotate_left(1).wrapping_add(17);
        } else {
            ws.quality = ws.quality.saturating_add(19);
            ws.energy = ws.energy.rotate_right(2).wrapping_add(23);
        }

        // ========= ここから「別PDAでの Bump Canonicalization 再発」部位 =========
        // 本来：bench を使うなら #[account(seeds=[b"bench", owner, tag], bump)] で検証し、
        //       署名にも同一配列と検証で得た bump を使うべき。
        // 誤り：手動で seeds/bump を再構築し、user_bump を使ってしまう。
        //       これが検証と署名の分離・再計算を招き、Bump Seed Canonicalization を復活させる。

        require!(item_tag.as_bytes().len() <= 32, Errs::TagTooLong);

        // 手動導出: seeds = [b"bench", owner, item_tag]
        // 署名: user_bump を採用（検証由来の bump と一致保証がない）
        let manual_seeds = &[
            b"bench".as_ref(),
            ws.owner.as_ref(),
            item_tag.as_bytes(),
            core::slice::from_ref(&user_bump),
        ];

        // 参考: 実際の PDA を「このプログラムID」×「手動 seeds + user_bump」で計算
        let manual_pda = Pubkey::create_program_address(
            &[b"bench", ws.owner.as_ref(), item_tag.as_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(Errs::SeedCompute))?;

        // manual_pda から recipient へLamports送金（System Program）
        // bench_hint は unchecked で、#[account(seeds=...)] による検証を受けていない
        let ix = system_instruction::transfer(&manual_pda, &ctx.accounts.recipient.key(), amount);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.bench_hint.to_account_info(),      // ← manual_pda に対応する口座を差し替え可能
                ctx.accounts.recipient.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[manual_seeds], // ← ここで user_bump を使って署名している
        )?;

        // 後処理も少し長めに：for → if
        for k in 0..6 {
            ws.energy = ws.energy.wrapping_add((k as u64) * 13 + 7);
            ws.quality = ws.quality.saturating_add(((amount % 17) as u32) + k as u32 + 4);
        }

        if ws.quality & 1 == 1 {
            ws.energy = ws.energy.rotate_left(2).wrapping_add(27);
            ws.quality = ws.quality.saturating_add(11);
        } else {
            ws.energy = ws.energy.rotate_right(1).wrapping_add(25);
            ws.quality = ws.quality.saturating_add(15);
        }

        Ok(())
    }
}

/* ---------------- Accounts ---------------- */

#[derive(Accounts)]
pub struct SetupWorkshop<'info> {
    // ここは Anchor が seeds/bump を検証（安全）
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4 + 1, // discriminator + owner + energy + quality + workshop_bump
        seeds = [b"workshop", owner.key().as_ref()],
        bump
    )]
    pub workshop: Account<'info, WorkshopState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CraftAndWithdraw<'info> {
    // workshop は安全に検証（ここは OK）
    #[account(
        mut,
        seeds = [b"workshop", owner.key().as_ref()],
        bump = workshop.workshop_bump
    )]
    pub workshop: Account<'info, WorkshopState>,

    // bench は検証していない（手動 seeds で署名するため差し替え余地がある）
    /// CHECK
    pub bench_hint: AccountInfo<'info>,

    /// CHECK: 受取人
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/* ---------------- State ---------------- */

#[account]
pub struct WorkshopState {
    pub owner: Pubkey,
    pub energy: u64,
    pub quality: u32,
    pub workshop_bump: u8,
}

/* ---------------- Errors ---------------- */

#[error_code]
pub enum Errs {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
    #[msg("tag too long")] TagTooLong,
}
