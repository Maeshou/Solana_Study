#!/usr/bin/env python3
import os
import sys

# コマンドライン引数で保存先ディレクトリを受け取る（指定がなければカレントディレクトリ）
if len(sys.argv) > 1:
    target_dir = sys.argv[1]
else:
    target_dir = "."

# 保存先ディレクトリが存在しない場合は作成する
os.makedirs(target_dir, exist_ok=True)
print("ファイルの保存先:", os.path.abspath(target_dir))

# 各ファイル名とその内容を辞書形式で定義
files = {
    "vulnerable_set_const.rs": r'''use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_set_const {
    use super::*;
    pub fn set_const(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：先頭バイトに42を書き込む
        ctx.accounts.account.data.borrow_mut()[0] = 42;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
''',

    "vulnerable_double_value.rs": r'''use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_double_value {
    use super::*;
    pub fn double_value(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：最初のバイトの値を2倍して上書きする
        let val = ctx.accounts.account.data.borrow()[0];
        ctx.accounts.account.data.borrow_mut()[0] = val.wrapping_mul(2);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
''',

    "vulnerable_bitwise_negate.rs": r'''use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_bitwise_negate {
    use super::*;
    pub fn bitwise_negate(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：3番目のバイトのビット反転（NOT）を行う
        let byte = ctx.accounts.account.data.borrow()[2];
        ctx.accounts.account.data.borrow_mut()[2] = !byte;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
''',

    "vulnerable_fill_pattern.rs": r'''use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_fill_pattern {
    use super::*;
    pub fn fill_pattern(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：最初の3バイトに同じパターン（値1）を書き込む
        let mut data = ctx.accounts.account.data.borrow_mut();
        data[0] = 1;
        data[1] = 1;
        data[2] = 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
''',

    "vulnerable_add_literal.rs": r'''use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_add_literal {
    use super::*;
    pub fn add_literal(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：4番目のバイトに 3 と 7 の和（10）を設定する
        ctx.accounts.account.data.borrow_mut()[3] = 3 + 7;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
''',

    "vulnerable_bitshift.rs": r'''use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_bitshift {
    use super::*;
    pub fn bitshift(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：2番目のバイトを左に1ビットシフトする
        let val = ctx.accounts.account.data.borrow()[1];
        ctx.accounts.account.data.borrow_mut()[1] = val << 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
''',

    "vulnerable_store_u32.rs": r'''use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_store_u32 {
    use super::*;
    pub fn store_u32(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：定数0xDEADBEEFをリトルエンディアン形式で先頭4バイトに格納する
        let value: u32 = 0xDEADBEEF;
        let mut data = ctx.accounts.account.data.borrow_mut();
        data[0] = (value & 0xFF) as u8;
        data[1] = ((value >> 8) & 0xFF) as u8;
        data[2] = ((value >> 16) & 0xFF) as u8;
        data[3] = ((value >> 24) & 0xFF) as u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
''',

    "vulnerable_modulo.rs": r'''use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_modulo {
    use super::*;
    pub fn modulo_value(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：最初のバイトの値を5で割った余りをその位置に再設定する
        let val = ctx.accounts.account.data.borrow()[0];
        ctx.accounts.account.data.borrow_mut()[0] = val % 5;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
''',

    "vulnerable_multiply.rs": r'''use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_multiply {
    use super::*;
    pub fn multiply_byte(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：最初のバイトの値に3を掛けて更新する
        let val = ctx.accounts.account.data.borrow()[0];
        ctx.accounts.account.data.borrow_mut()[0] = val.wrapping_mul(3);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
''',

    "vulnerable_sum_literals.rs": r'''use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod vulnerable_sum_literals {
    use super::*;
    pub fn sum_literals(ctx: Context<UpdateData>) -> ProgramResult {
        // オーナーチェックなし：2番目のバイトに10と20の和（30）を設定する
        ctx.accounts.account.data.borrow_mut()[1] = 10 + 20;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub account: AccountInfo<'info>,
}
'''
}

# 各ファイルを指定した保存先ディレクトリに作成する
for filename, content in files.items():
    file_path = os.path.join(target_dir, filename)
    try:
        with open(file_path, "w", encoding="utf-8") as f:
            f.write(content)
        print(f"{file_path} を作成しました。")
    except Exception as e:
        print(f"{file_path} の作成中にエラーが発生しました: {e}")
