use anchor_lang::prelude::*;
declare_id!("G1u1dLedGer111111111111111111111111111111");

#[program]
pub mod guild_ledger {
    use super::*;
    use Role::*;

    pub fn init_guild(ctx: Context<InitGuild>, name: String) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.authority = ctx.accounts.authority.key();
        g.name = name;
        g.total_points = 0;
        Ok(())
    }

    pub fn enroll_member(ctx: Context<EnrollMember>, role: Role, tag: u8) -> Result<()> {
        let card = &mut ctx.accounts.member_card;
        card.parent = ctx.accounts.guild.key();
        card.role = role;
        card.tag = tag;
        card.points = 0;
        Ok(())
    }

    pub fn update_board(ctx: Context<UpdateBoard>, delta: i64) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        let actor = &mut ctx.accounts.member_actor;
        let counter = &mut ctx.accounts.member_counter;
        let board = &mut ctx.accounts.board;

        // 移動平均 + しきい値判定 + ビット操作
        let mut sum: i64 = 0;
        for i in 0..board.slots.len() {
            let base = board.slots[i] as i64;
            let adj = (base ^ (delta as i64)) & 0x7FFF;
            let capped = adj.clamp(0, 50_000);
            board.slots[i] = capped as u32;
            sum = sum.saturating_add(capped);
        }
        let avg = if board.slots.len() > 0 { sum / board.slots.len() as i64 } else { 0 };

        if avg >= 10_000 {
            // 実績大 → actor を厚遇
            actor.points = actor
                .points
                .checked_add((avg as u64).min(5_000))
                .unwrap_or(u64::MAX);
            guild.total_points = guild.total_points.saturating_add(actor.points / 10);
            board.flag = true;
            msg!("High performance: actor boosted, guild total updated");
        } else {
            // 実績小 → counter を底上げ
            let boost = (10_000 - avg).max(0) as u64;
            counter.points = counter.points.checked_add(boost).unwrap_or(u64::MAX);
            guild.total_points = guild.total_points.saturating_add(counter.points / 20);
            board.flag = false;
            msg!("Low performance: counter supported, guild total updated");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 4 + 64 + 8)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnrollMember<'info> {
    #[account(mut, has_one = authority)]
    pub guild: Account<'info, Guild>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 1 + 8
    )]
    pub member_card: Account<'info, MemberCard>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBoard<'info> {
    // 親: 自プログラム所有（Account<T> が discriminator + owner を保証）
    #[account(mut, has_one = authority)]
    pub guild: Account<'info, Guild>,

    // 子: 親リンク必須 & 役割不一致 & 同一口座二重渡し阻止
    #[account(
        mut,
        constraint = member_actor.parent == guild.key() @ ErrCode::CosplayBlocked,
        constraint = (member_actor.role as u8) != (member_counter.role as u8) @ ErrCode::CosplayBlocked,
        constraint = member_actor.key() != member_counter.key() @ ErrCode::CosplayBlocked
    )]
    pub member_actor: Account<'info, MemberCard>,

    #[account(
        mut,
        constraint = member_counter.parent == guild.key() @ ErrCode::CosplayBlocked
    )]
    pub member_counter: Account<'info, MemberCard>,

    #[account(
        mut,
        constraint = board.parent == guild.key() @ ErrCode::CosplayBlocked
    )]
    pub board: Account<'info, Board>,

    pub authority: Signer<'info>,
}

#[account]
pub struct Guild {
    pub authority: Pubkey,
    pub name: String,   // 最大64文字想定（Init時のspaceに反映）
    pub total_points: u64,
}

#[account]
pub struct MemberCard {
    pub parent: Pubkey, // = guild
    pub role: Role,
    pub tag: u8,
    pub points: u64,
}

#[account]
pub struct Board {
    pub parent: Pubkey,     // = guild
    pub slots: [u32; 8],    // 32 bytes
    pub flag: bool,         // 1 byte
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Role { Leader, Raider, Crafter, Scout }

#[error_code]
pub enum ErrCode {
    #[msg("Type cosplay blocked by role mismatch / link checks")]
    CosplayBlocked,
}
