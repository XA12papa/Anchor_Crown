
// use std::assert_eq;

// use anchor_lang::{ AccountDeserialize, InstructionData, solana_program::{self, feature::instruction}};
// use litesvm::LiteSVM;
// use solana_keypair::Keypair;
// use solana_signer::Suse std::{assert_eq};

use std::assert_eq;

use anchor_lang::{ AccountDeserialize, InstructionData, solana_program};
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_instruction::{AccountMeta,Instruction};
use solana_message::Message;
use solana_transaction::Transaction;
use solana_pubkey::Pubkey;


#[test]
fn test_update() {
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(
        crown::ID, 
        "../../target/deploy/crown.so" // path to the .so file 
     ).unwrap();


    //client id 
    let instructor = Keypair::new();     
    svm.airdrop(
        &instructor.pubkey(),
        1_000_000_000,
    ).unwrap();

    let (course_pda, _bump) = Pubkey::find_program_address(
        &[
            b"course", // the seed is a byte string that is used to generate the PDA
            instructor.pubkey().as_ref(), // the seed is the instructor's public key
            &1u64.to_le_bytes() // the seed is the course id in little endian format
        ],
        &crown::ID,
    );

    let data  = crown::instruction::InitializeCourse{
        course_id: 1,
        title : "Solana Developmet".to_string(),
        price : 1_000_000_000,
    }.data();


    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(course_pda, false),
        AccountMeta::new(instructor.pubkey(),true),
        AccountMeta::new_readonly(solana_program::system_program::ID, false),
    ];

    let instruction = Instruction {
        program_id: crown::ID,
        accounts,
        data
    };

    // the message is transactions execution plans before signatures are attached 
    // solana needs message because it wants to know all the accouts it need upfront

    // example
    //  CROWN instruction
    //    │
    //    ├── Course
    //    ├── Instructor
    //    └── System Program
    let message =  Message::new(
        &[
            // istruction1
            // istruction2
            // istruction3
            instruction
            ],

        Some(&instructor.pubkey()), // the fee payer of the transaction not nesseraly the feepayer of the accout that will be created 

    let tx = Transaction::new(
        &[&instructor ], // list of all the signers that are required here
         message, 
        svm.latest_blockhash()
    );

    svm.send_transaction(tx).unwrap();





    //  update instructions 
        // course_id: 1,
        // title : "How make her cum".to_string(),
        // price : 2_000_000_000,

    let data  = crown::instruction::UpdateCourse{
        _course_id : 1 ,
        new_title : "How make her cum".to_string() ,
        new_price : 2_000_000_000
    }.data();


    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(course_pda, false),
        AccountMeta::new_readonly(instructor.pubkey(),true),
    ];

    let instruction = Instruction {
        program_id: crown::ID,
        accounts,
        data
    };

    let message =  Message::new(
        &[instruction],

        Some(&instructor.pubkey()), // the fee payer of the transaction not nesseraly the feepayer of the accout that will be created 
    );


    let tx = Transaction::new(
        &[&instructor ], // list of all the signers that are required here
         message, 
        svm.latest_blockhash()
    );


    svm.send_transaction(tx).unwrap();



    //  testing the  datas ;
    let account = svm
    .get_account(&course_pda)
    .unwrap();

    let mut data :&[u8] = &account.data;

    let course_data = crown::Course::try_deserialize(
        &mut data
    ).unwrap();

    assert_ne!(
        course_data.price,
        2_000_000_000
    );
    assert_ne!(course_data.title,"Solana Developmet");






    // Add your test logic here
}
