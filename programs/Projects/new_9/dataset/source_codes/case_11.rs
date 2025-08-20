use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("ReviveDemo1111111111111111111111111111111");

#[program]
pub mod revival_examples {
    use super::*;

    // ① 正しく "close 属性" を付けているが…
    pub fn close_scroll(ctx: Context<CloseScroll>) -> Result<()> {
        // 命令終了時に Anchor が `scroll` の lamports を `collector` へ送る
        Ok(())
    }

    // ② 同じトランザクション中の別命令で、同じアドレスを SystemProgram で再初期化
    //    * 口座は GC されておらず存在しているため、lamports を戻し、allocate→assign→（自前初期化）で復活可能
    pub fn revive_scroll_same_tx(
        ctx: Context<ReviveScrollSameTx>,
        space: u64,
        seed_value: u64,
    ) -> Result<()> {
        let scroll_ai = ctx.accounts.scroll_addr.to_account_info();
        let payer_ai = ctx.accounts.payer.to_account_info();

        // lamports を戻す（送金のみ）
        let restore = system_instruction::transfer(&payer_ai.key(), &scroll_ai.key(), 1_000_000);
        anchor_lang::solana_program::program::invoke(
            &restore,
            &[payer_ai.clone(), scroll_ai.clone()],
        )?;

        // allocate（データ長を付与）
        let alloc = system_instruction::allocate(&scroll_ai.key(), space);
        anchor_lang::solana_program::program::invoke(&alloc, &[scroll_ai.clone()])?;

        // assign（所有者をこのプログラムに設定）
        let assign = system_instruction::assign(&scroll_ai.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(&assign, &[scroll_ai.clone()])?;

        // 以降、任意の初期化ロジック（例：seed_value を書き込む）
        let mut data = scroll_ai.try_borrow_mut_data()?;
        bytemuck::bytes_of(&seed_value)
            .iter()
            .enumerate()
            .for_each(|(i, b)| data[i] = *b);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseScroll<'info> {
    // ← 命令終了時に Anchor が close を実施
    #[account(mut, close = collector)]
    pub scroll: Account<'info, ScrollData>,
    /// CHECK: 受け取り先
    #[account(mut)]
    pub collector: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ReviveScrollSameTx<'info> {
    /// CHECK: 先ほど close したのと同じアドレスを「単なるアカウント情報」として受け取っている
    #[account(mut)]
    pub scroll_addr: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ScrollData {
    pub marker: u64,
}
