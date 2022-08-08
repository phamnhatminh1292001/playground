use std::vec;
use vrt::dvm::DVM;

fn main() {
    // 45022 - ((86 + 119)*34)/2
    // Example program in DVM opcode!
    // PUSH 0x00000056     0x05, 0x00, 0x00, 0x00, 0x56
    // PUSH 0x00000077     0x05, 0x00, 0x00, 0x00, 0x77
    // ADD                 0x01
    // PUSH 0x00000022     0x05, 0x00, 0x00, 0x00, 0x22
    // MUL                 0x03
    // PUSH 0x00000002     0x05, 0x00, 0x00, 0x00, 0x02
    // DIV                 0x04
    // PUSH 0x0000afde     0x05, 0x00, 0x00, 0xaf, 0xde
    // SWAP                0x08
    // SUB                 0x02
    // RET                 0x07
    let program = vec![
        0x05u8, 0x00, 0x00, 0x00, 0x56, 0x05, 0x00, 0x00, 0x00, 0x77, 0x01, 0x05, 0x00, 0x00, 0x00,
        0x22, 0x03, 0x05, 0x00, 0x00, 0x00, 0x02, 0x04, 0x05, 0x00, 0x00, 0xaf, 0xde, 0x08, 0x02,
        0x05, 0x00, 0x12, 0xae, 0x24, 0x05, 0x00, 0x11, 0x0e, 0x12, 0x01, 0x05, 0x00, 0x23, 0x45,
        0x23, 0x02, 0x07,
    ];
    let mut my_dummy_vm = DVM::new();
    println!("Result: {}", my_dummy_vm.process(program));
}
