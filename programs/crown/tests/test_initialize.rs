use std::{assert_eq};

use anchor_lang::{AccountDeserialize, InstructionData, solana_program};
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_instruction::{AccountMeta,Instruction};
use solana_message::Message;
use solana_transaction::Transaction;
use solana_pubkey::Pubkey;

#[test]
fn test_initialize() {
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

    assert_eq!(
        course_data.instructor,
        instructor.pubkey()
    );

    assert_eq!(
        course_data.price,
        1_000_000_000
    );

        assert_eq!(
        course_data.title,
        "Solana Developmet"
    );

    assert_eq!(
        course_data.course_id,
        1
    );



    assert_eq!(
        course_data.bump,
        _bump // Replace with the actual bump value
    );

    assert_eq!(
        course_pda,
        Pubkey::find_program_address(
            &[
                b"course", // the seed is a byte string that is used to generate the PDA
                instructor.pubkey().as_ref(), // the seed is the instructor's public key
                &1u64.to_le_bytes() // the seed is the course id in little endian format
            ],
            &crown::ID,
        ).0
    );
    // Add your test logic here
}


#[test]
fn delebrate_pda_fail(){
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
            &10u64.to_le_bytes() // the seed is the course id in little endian format
        ],
        &crown::ID,
    );

    let data  = crown::instruction::InitializeCourse{
        course_id: 1, // here the seed input is wrong 
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
    );


    let tx = Transaction::new(
        &[&instructor ], // list of all the signers that are required here
         message, 
        svm.latest_blockhash()
    );

    svm.send_transaction(tx).unwrap();


}

// #[test]
// fn test_initialize_failure_title_empty() {
//     let mut svm = LiteSVM::new();
//     svm.add_program_from_file(
//         crown::ID, 
//         "../../target/deploy/crown.so" // path to the .so file 
//      ).unwrap();


//     //client id 
//     let instructor = Keypair::new();
//     let course = Keypair::new();
     

//     svm.airdrop(
//         &instructor.pubkey(),
//         1_000_000_000,
//     ).unwrap();

//     let data  = crown::instruction::InitializeCourse{
//         title : "".to_string(),
//         price : 1_000_000_000,
//     }.data();


//     let accounts: Vec<AccountMeta> = vec![
//         AccountMeta::new(course.pubkey(), true),
//         AccountMeta::new(instructor.pubkey(),true),
//         AccountMeta::new_readonly(solana_program::system_program::ID, false),
//     ];

//     let instruction = Instruction {
//         program_id: crown::ID,
//         accounts,
//         data
//     };

//     // the message is transactions execution plans before signatures are attached 
//     // solana needs message because it wants to know all the accouts it need upfront

//     // example
//     //  CROWN instruction
//     //    │
//     //    ├── Course
//     //    ├── Instructor
//     //    └── System Program
//     let message =  Message::new(
//         &[
//             // istruction1
//             // istruction2
//             // istruction3
//             instruction
//             ],

//         Some(&instructor.pubkey()), // the fee payer of the transaction not nesseraly the feepayer of the accout that will be created 
//     );


//     let tx = Transaction::new(
//         &[&instructor, &course ],
//          message, 
//         svm.latest_blockhash()
//     );

//     // svm.send_transaction(tx).unwrap();

//     let result = svm.send_transaction(tx);

//     match result {
//         Ok(_) => panic!("This should have failed"),
//         Err(err) =>{
//             let error =  format!("{err:#?}");

//             assert!(
//                 error.contains("EmptyTitle")
//                 || error.contains("Course title cannot be empty.")
//             );
//         }
//     }


// }



// // IMPORTANT ARCHETECTURE 
//         //           initialize_course
//         //                  │
//         //                  ▼
//         //         Account validation
//         //                  │
//         //     ┌────────────┴────────────┐
//         //     │                         │
//         //  invalid                    valid
//         //     │                         │
//         //     ▼                         ▼
//         //   ERROR                    handler
//         //                               │
//         //                               ▼
//         //                       Business validation
//         //                               │
//         //                       ┌───────┴───────┐
//         //                       │               │
//         //                    invalid          valid
//         //                       │               │
//         //                       ▼               ▼
//         //                     ERROR          state update