use std::{assert_eq, println};

use anchor_lang::{ AccountDeserialize, InstructionData, solana_program};
use crown::accounts;
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_instruction::{AccountMeta,Instruction};
use solana_message::Message;
use solana_transaction::Transaction;
use solana_pubkey::Pubkey;


#[test] 
fn test_terminate(){
    let mut svm  = LiteSVM::new();
    svm.add_program_from_file(
        crown::ID, 
        "../../target/deploy/crown.so"
    ).unwrap();


    let instructor = Keypair::new();
    svm.airdrop(
        &instructor.pubkey(),
        1_000_000_000,
    ).unwrap();

    let initial_balance =  svm.get_balance(&instructor.pubkey()).unwrap();

    println!("Initial Balance : {}", initial_balance);

    let (course_pda, _bump) = Pubkey::find_program_address(
        &[
            b"course",
            instructor.pubkey().as_ref(),
            &1u64.to_le_bytes()
        ]
        , &crown::ID
    );

    let data = crown::instruction::InitializeCourse{
        course_id:1,
        title : "Intel".to_string(),
        price : 1_000_000_000,
    }.data();

    let accounts : Vec<AccountMeta>  = vec![
        AccountMeta::new(course_pda,false),
        AccountMeta::new(instructor.pubkey(),true),
        AccountMeta::new_readonly(solana_program::system_program::ID,false)
    ];

    let instruction =  Instruction{
        program_id : crown::ID,
        accounts,
        data
    };

     let message = Message::new(
        &[instruction],
            Some(&instructor.pubkey())
    );

    let tx = Transaction::new(
        
        &[&instructor],
        message,
        svm.latest_blockhash()
    );

    svm.send_transaction(tx).unwrap();


    let after_creation =  svm.get_balance(&instructor.pubkey()).unwrap();

    println!("Balance after account creation : {}", after_creation);

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


    let data = crown::instruction::TerminateCourse{
        course_id: 1,
    }.data();

    let accounts : Vec<AccountMeta>  = vec![
        AccountMeta::new(course_pda,false),
        AccountMeta::new(instructor.pubkey(),true),
    ];

    let instruction =  Instruction{
        program_id : crown::ID,
        accounts,
        data
    };

    let message = Message::new(
        &[instruction],
            Some(&instructor.pubkey())
    );


    let tx = Transaction::new(
        
        &[&instructor],
        message,
        svm.latest_blockhash()
    );

    svm.send_transaction(tx).unwrap();

    let response = svm
    .get_account(&course_pda)
    ;

    match response {
       Some(account)  =>{
            let mut data :&[u8] = &account.data;

                let course_data = crown::Course::try_deserialize(
                    &mut data
                ).unwrap();

                println!("course ID : ===> {:?}", course_data.course_id);
       }
       None => {
            let after_termination =  svm.get_balance(&instructor.pubkey()).unwrap();

            println!("Balance after account creation : {}", after_termination);
       }
    }

    
}