use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define el tipo de estado almacenado en las cuentas
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BuyAccount {
    /// número de libros comprados
    pub counter: u32,
}

// Declarar y exportar el punto de entrada del programa
entrypoint!(process_instruction);

// Implementación del punto de entrada del programa
pub fn process_instruction(
    // Clave pública de la cuenta en la que se cargó el programa de libros
    program_id: &Pubkey,
    // La cuenta en la que se va a comprar el libro
    accounts: &[AccountInfo],
    // Ignorado, todas las instrucciones son compras
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Nuevo libro");

    // Iterar las cuentas es más seguro que indexarlas
    let accounts_iter = &mut accounts.iter();

    // Obtener la cuenta en la que se va a comprar
    let account = next_account_info(accounts_iter)?;

    // La cuenta debe ser propiedad del programa para poder modificar sus datos
    if account.owner != program_id {
        msg!("La cuenta de compra no tiene la identificación de programa correcta");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Incrementar y almacenar la cantidad de veces que se ha comprado en la cuenta
    let mut buy_account = BuyAccount::try_from_slice(&account.data.borrow())?;
    buy_account.counter += 1;
    buy_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("¡Comprado {} vez(es)!", buy_account.counter);

    Ok(())
}

// Pruebas de integridad
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        // Preparar datos de prueba
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        // Crear un vector de cuentas para la prueba
        let accounts = vec![account];

        // Asegurarse de que el contador de compras comienza en 0
        assert_eq!(
            BuyAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        // Llamar al programa una vez
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        // Verificar que el contador de compras se incrementa a 1
        assert_eq!(
            BuyAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );

        // Llamar al programa nuevamente
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        // Verificar que el contador de compras se incrementa a 2
        assert_eq!(
            BuyAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}