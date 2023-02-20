/**************************************************************************************************
* Name : 									   asm.rs
* Author : 										Avery
* Date : 									  2/04/2023
* Purpose : 					          Assembly functions
* Version : 									 0.1
**************************************************************************************************/

use core::arch::asm;

pub fn test_asm() { 
    let mut result: u64 = 0;

    unsafe {
        // move 10 into rax and 2 into rbx and multiply them
        asm!("mov rax, 10
              mov rbx, 2
              mul rbx",
             out("rax") result);
    }

    assert_eq!(result, 20, "Assembly test failed! Expected: 20, Result: {}", result);
}

pub fn triple_fault() {
    unsafe {
        /********************************************************************
        * load interrupt descriptor table at 0xdead, then trigger interrupt 3
        ********************************************************************/
        asm!(
            "lidt [rax]
            int 3",
        in("rax") 0xdead);
    }
}