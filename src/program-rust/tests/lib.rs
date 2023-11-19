/// Importación de bibliotecas necesarias para el la realizacion del test
/// Importa las macros de serialización Y deserialización de Bors
use borsh::BorshDeserialize;
/// Importa los elementos de prueba
use helloworld::{process_instruction, GreetingAccount};
/// Importa los elementos publicos del test
use solana_program_test::*;
/// Importa los elemtos nesesarios para realizar las pruebas, tal como la estructura de cuentras, las instrucciones, las claves, las firmas y las transacciones
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
/// importa las librerias necesarias para interactuar con el sistema y la memoria del dispositivo
use std::mem;
/// establece el runtime de tokio para la prueba
/// para solana se toma como un estandar tokio
#[tokio::test]
/// declara el test basico
async fn test_AppleStockSim() {
    /// Crea una nueva clave publica para la prueba
    let program_id = Pubkey::new_unique();
    /// Crea una cuenta usuario
    let reviewed_pubkey = Pubkey::new_unique();
    /// Inicia la instancia de prueba
    let mut program_test = ProgramTest::new(
        "AppleStockSim",
        program_id,
        processor!(process_instruction),
    );
    /// Agrega una cuenta al programa de prueba
    program_test.add_account(
        reviewed_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );
    /// Obtiene los datos nesesarios para realizar la interaccion con el programa
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    /// valida las condiciones de inicialización del programa, counter debe ser 0
    let reviewed_account = banks_client
        .get_account(reviewed_pubkey)
        .await
        .expect("get_account")
        .expect("reviewed_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&reviewed_account.data)
            .unwrap()
            .counter,
        0
    );

    /// Crea la transacción de prueba
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(reviewed_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    /// Firma de la transacción
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    /// Verifica el resultado de la transacción, ahora count debe ser 1
    let verify_account = banks_client
        .get_account(reviewed_pubkey)
        .await
        .expect("get_account")
        .expect("verify_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&verify_account.data)
            .unwrap()
            .counter,
        1
    );

    /// Realiza un nuevo llamado de prueba
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[1], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(reviewed_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    /// Firma y envia la transacción
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    /// Verifica el resultado de la transacción, counter ahora deber ser 2
    let verify_two_account = banks_client
        .get_account(reviewed_pubkey)
        .await
        .expect("get_account")
        .expect("verify_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&verify_two_account.data)
            .unwrap()
            .counter,
        2
    );
}
