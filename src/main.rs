use core::arch::naked_asm;

#[unsafe(naked)]
pub unsafe extern "C" fn server_ui_handler() {
    naked_asm!(
        // 1. 保存寄存器 (Context Save)
        "push rax",
        "push rcx",
        "push rdx",
        // ... 保存其他通用寄存器 ...

        // 2. 调用实际的逻辑函数
        "call {logic_func}",

        // 3. 恢复寄存器 (Context Restore)
        // ... pop ...
        "pop rdx",
        "pop rcx",
        "pop rax",

        // 4. 使用 uiret 返回
        "uiret",
        logic_func = sym actual_logic_in_rust,
    );
}

fn actual_logic_in_rust() {}

fn main() {}
