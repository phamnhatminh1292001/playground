use crate::proofs::high_level_plain_proof::high_level_plain_proof::HighLevelPlainProof;
use crate::runtime::execution::Execution;
use crate::runtime::opcode_util::opcode_with_params::OpcodeWithParams;
use crate::test::programs::collatz_program::{make_collatz_program_memory, NUM_COLLATZ_INPUTS};
use crate::test::programs::gcd_program::{make_gcd_program_memory, NUM_GCD_INPUTS};
use strum::IntoEnumIterator;

use crate::runtime::dvm::DummyVirtualMachine;
use crate::runtime::error_code_util::error_code::ErrorCode;
use crate::utils::numeric_encoding::NumericEncoding;


fn test_with_program_memory(num_steps: usize, program_memory: &Vec<OpcodeWithParams>, input: &[u32]) {
    let mut dummy_vm = DummyVirtualMachine::new(program_memory);

    dummy_vm.get_program_memory().display();
        
        println!("Possible errors: ");
        for element in ErrorCode::iter() {
            println!("{}: {:?}", element.to_u32(), element);
        }

        let (result, error_code, execution_trace) = dummy_vm.execute(num_steps);
        println!("Input = {:?}, Result = {}, Error Code = {:?}", input, result, error_code);

        //print_vector(&format!("Directions ({} elements): ", direction_trace.len()), direction_trace, ',');

        // let program_counter_trace = execution_trace.get_program_counter_trace();
        // print_vector(&format!("Program counters ({} elements): ", program_counter_trace.len()), program_counter_trace, ',');

        // let stack_trace = execution_trace.get_stack_trace();
        // print_vector(&format!("Stack trace ({} elements): ", stack_trace.len()), stack_trace, '\n');

        let high_level_plain_proof = HighLevelPlainProof::new(&execution_trace);
        high_level_plain_proof.verify();
} 

pub fn do_test(num_steps: usize) {
    let gcd_test_vector: Vec<[u32; NUM_GCD_INPUTS]> = vec![
        [0, 0],
        [10, 0], 
        [0, 4],
        [4, 12],
        [20, 100],
        [15, 7],
        [324, 2442],
    ];

    for input in gcd_test_vector {
        test_with_program_memory(num_steps, &make_gcd_program_memory(&input), &input);
    }

    let collatz_test_vector: Vec<[u32; NUM_COLLATZ_INPUTS]> = vec![
        [0],
        [1],
        [2],
        [3],
        [4],
        [5],
        [6],
        [10],
        [1000],
    ];

    for input in collatz_test_vector {
        test_with_program_memory(num_steps, &make_collatz_program_memory(&input), &input);
    }
}