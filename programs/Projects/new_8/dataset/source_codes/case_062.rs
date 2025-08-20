use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction, pubkey::Pubkey};

declare_id!("SeEdShUf2222222222222222222222222222222");

#[program]
pub mod shuffle_logic_case {
    use super::*;

    pub fn make_box(ctx: Context<MakeBox>, count: u64) -> Result<()> {
        let b = &mut ctx.accounts.box_data;
        b.owner = ctx.accounts.owner.key();
        b.bump_saved = *ctx.bumps.get("box_data").ok_or(error!(Errs::MissingBump))?;
        b.count = count % 55 + 5;
        b.tally = 0;

        // ループと分岐を交互に挿入して状況を攪拌
        let mut rolling = b.count ^ (Clock::get()?.slot as u64);
        for step in 0..7 {
            if step % 2 == 0 {
                b.tally = b.tally.saturating_add(((rolling as u32) ^ (step as u32)).wrapping_add(13));
            }
            rolling = rolling.rotate_left((step + 3) as u32).wrapping_add(step as u64);
        }

        Ok(())
    }

    // 検証で使った seeds/bump（[b"box", owner] + Anchorのbump）と
    // 署名で使う seeds/bump を意図的にズラしたパターン。
    // * prefix を "box_shadow" に変更
    // * bump_saved に lamports 下位ビットを混ぜて別 bump を使用
    // * さらにオーダー違い seeds を用意して二段階で署名
    pub fn withdraw(ctx: Context<Withdraw>, lamports: u64) -> Result<()> {
        let meta = &ctx.accounts.box_data;

        // 1) “保存済み bump” に動的オフセットを混ぜて別 PDA を導出
        let bump_variant = meta.bump_saved.wrapping_add(((lamports & 3) as u8) ^ 2);
        let seeds_primary: &[&[u8]] = &[
            b"box_shadow",
            meta.owner.as_ref(),
            &[bump_variant],
        ];

        let pda_primary = Pubkey::create_program_address(
            &[b"box_shadow", meta.owner.as_ref(), &[bump_variant]],
            ctx.program_id,
        ).map_err(|_| error!(Errs::SeedCompute))?;

        // alt_box を “box_shadow/owner/bump_variant” の PDA とみなして送金命令を作る
        let ix1 = system_instruction::transfer(
            &pda_primary,
            &ctx.accounts.target.key(),
            lamports.saturating_add(97),
        );

        let infos1 = &[
            ctx.accounts.alt_box.to_account_info(),
            ctx.accounts.target.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];

        // 署名に使う seeds が検証と不一致（canonicalize されていない）
        invoke_signed(&ix1, infos1, &[seeds_primary])?;

        // 2) さらに seeds の順序を入れ替えた “別の” 導出で追撃（検証と二重に乖離）
        //    ※ 実運用では create_program_address と一致しない場合があるが、
        //       ここでは “ズレた seeds で署名を試みる” パターン自体を示す。
        let bump_swapped = bump_variant.wrapping_add(1);
        let seeds_swapped: &[&[u8]] = &[
            meta.owner.as_ref(),           // ← 順序を入れ替え
            b"box_shadow",
            &[bump_swapped],
        ];

        let _maybe_swapped = Pubkey::create_program_address(
            &[meta.owner.as_ref(), b"box_shadow", &[bump_swapped]],
            ctx.program_id,
        ).unwrap_or(meta.owner); // 失敗しても続行（ズレの存在を示すため）

        let ix2 = system_instruction::transfer(
            &ctx.accounts.alt_box.key(),   // alt_box をそのまま from に使う別経路
            &ctx.accounts.target.key(),
            (lamports ^ 41).saturating_add(11),
        );
        let infos2 = &[
            ctx.accounts.alt_box.to_account_info(),
            ctx.accounts.target.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        invoke_signed(&ix2, infos2, &[seeds_swapped])?;

        // 3) 状態更新を複層的に（短すぎないロジック）
        let mut remain = lamports.wrapping_add((Clock::get()?.unix_timestamp as u64) & 127);
        let mut loop_guard = 0u8;
        while loop_guard < 4 {
            let mix = (remain % 29) as u32;
            ctx.accounts.box_data.tally = ctx.accounts.box_data.tally
                .wrapping_add(mix.rotate_left((loop_guard + 3) as u32))
                .wrapping_add(17);
            remain = remain.rotate_right(2).wrapping_add(5);
            loop_guard = loop_guard.saturating_add(1);
        }

        if (remain & 1) == 1 {
            // 偶奇で別処理：ビット演算と小ループを併用
            let mut spice = 3u32;
            for k in 0..3 {
                spice = spice.rotate_left(k + 2).wrapping_mul(7).wrapping_add(k as u32);
            }
            ctx.accounts.box_data.tally = ctx.accounts.box_data.tally.wrapping_add(spice);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MakeBox<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4 + 1,
        seeds = [b"box", owner.key().as_ref()],
        bump
    )]
    pub box_data: Account<'info, BoxData>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // 検証は “box/owner/anchor-bump”
    #[account(mut, seeds = [b"box", owner.key().as_ref()], bump)]
    pub box_data: Account<'info, BoxData>,

    /// CHECK: 署名経路では “box_shadow/owner/bump_variant” を想定して使われる
    #[account(mut)]
    pub alt_box: AccountInfo<'info>,

    /// CHECK: 受け取り先
    #[account(mut)]
    pub target: AccountInfo<'info>,

    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BoxData {
    pub owner: Pubkey,
    pub count: u64,
    pub tally: u32,
    pub bump_saved: u8,
}

#[error_code]
pub enum Errs {
    #[msg("required bump not found in bumps map")]
    MissingBump,
    #[msg("failed to derive PDA with provided seeds/bump")]
    SeedCompute,
}
