use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    clockwork_network_program::objects::{Fee, Penalty, Pool},
};

pub fn thread_crank(signatory: Pubkey, thread: Pubkey, worker: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new(Fee::pubkey(worker), false),
            AccountMeta::new(Penalty::pubkey(worker), false),
            AccountMeta::new_readonly(Pool::pubkey(0), false),
            AccountMeta::new(signatory, true),
            AccountMeta::new(thread, false),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_thread_program::instruction::ThreadCrank {}.data(),
    }
}