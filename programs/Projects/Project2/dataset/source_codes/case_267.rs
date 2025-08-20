use anchor_lang::prelude::*;

declare_id!("QuestLink0666666666666666666666666666666666");

#[program]
pub mod quest_list {
    use super::*;

    pub fn init_list(ctx: Context<InitList>, first_id: u64) -> Result<()> {
        let ql = &mut ctx.accounts.qlist;
        ql.head = Some(Box::new(Node { id: first_id, next: None }));
        ql.count = 1;
        Ok(())
    }

    pub fn append(ctx: Context<InitList>, new_id: u64) -> Result<()> {
        let ql = &mut ctx.accounts.qlist;
        let mut current = &mut ql.head;
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(Box::new(Node { id: new_id, next: None }));
                ql.count = ql.count.saturating_add(1);
                break;
            }
            current = &mut node.next;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitList<'info> {
    #[account(init, payer = user, space = 8 + 8 + (8 + 1 + 32) * 10)]
    pub qlist: Account<'info, QuestList>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct QuestList {
    pub head: Option<Box<Node>>,
    pub count: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Node {
    pub id: u64,
    pub next: Option<Box<Node>>,
}
