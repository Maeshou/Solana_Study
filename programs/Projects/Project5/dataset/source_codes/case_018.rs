// ============================================================================
// 3) Relic Expedition（遺物探索）— PDA使用 + seeds固定 + constraint + assert_ne!
//    防止法: seedsでLog固定, 属性で相互≠, 一部は標準assertで二重可変ブロック
// ============================================================================
declare_id!("RLXC33333333333333333333333333333333");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ExpeditionFlag { Idle, Running }

#[program]
pub mod relic_expedition {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, need: u32) -> Result<()> {
        ctx.accounts.board.host = ctx.accounts.master.key();
        ctx.accounts.board.need = need;
        ctx.accounts.board.flag = ExpeditionFlag::Idle;

        ctx.accounts.party.leader = ctx.accounts.explorer.key();
        ctx.accounts.party.level = 1;
        ctx.accounts.party.ok = 1;

        ctx.accounts.log.steps = 0;
        ctx.accounts.log.loot = 0;
        Ok(())
    }

    pub fn explore(ctx: Context<Explore>, steps: u32) -> Result<()> {
        // 標準assert：boardとpartyは別、logとも別
        assert_ne!(ctx.accounts.board.key(), ctx.accounts.party.key(), "dup board/party");
        assert_ne!(ctx.accounts.board.key(), ctx.accounts.log.key(), "dup board/log");

        // ループ
        let mut s = 0;
        while s < steps {
            ctx.accounts.party.level = ctx.accounts.party.level.saturating_add(1);
            ctx.accounts.log.steps = ctx.accounts.log.steps.saturating_add(1);
            ctx.accounts.log.loot = ctx.accounts.log.loot.saturating_add(1);
            s += 1;
        }

        // 分岐
        if steps >= ctx.accounts.board.need {
            ctx.accounts.board.flag = ExpeditionFlag::Running;
            ctx.accounts.party.ok = 0;
            ctx.accounts.log.loot = ctx.accounts.log.loot.saturating_add(4);
            msg!("milestone reached; bonus loot");
        } else {
            ctx.accounts.board.flag = ExpeditionFlag::Idle;
            ctx.accounts.party.ok = 1;
            ctx.accounts.log.steps = ctx.accounts.log.steps.saturating_add(1);
            msg!("keep exploring; +1 step");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub board: Account<'info, Board>,
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub party: Account<'info, Party>,
    #[account(init, payer = payer, space = 8 + 4 + 8, seeds=[b"log", payer.key().as_ref()], bump)]
    pub log: Account<'info, ExpLog>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub master: Signer<'info>,
    pub explorer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Explore<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    #[account(mut, constraint = party.key() != log.key(), error = ExpErr::Same)]
    pub party: Account<'info, Party>,
    #[account(mut, seeds=[b"log", payer.key().as_ref()], bump)]
    pub log: Account<'info, ExpLog>,
    /// CHECK: seeds固定のためOK
    pub payer: UncheckedAccount<'info>,
}

#[account] pub struct Board { pub host: Pubkey, pub need: u32, pub flag: ExpeditionFlag }
#[account] pub struct Party { pub leader: Pubkey, pub level: u32, pub ok: u8 }
#[account] pub struct ExpLog { pub steps: u32, pub loot: u64 }

#[error_code] pub enum ExpErr { #[msg("dup")] Same }

