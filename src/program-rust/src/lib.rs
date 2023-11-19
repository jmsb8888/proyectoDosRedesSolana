/// Importación de bibliotecas necesarias para el programa Solana en Rust
/// Importa las macros de serialización Y deserialización de Borsh
use borsh::{BorshDeserialize, BorshSerialize};
/// Importa los módulos necesarios de solana_program
use solana_program::{
    /// Funciones y tipos relacionados con la manipulacion de las cuentas
    account_info::{next_account_info, AccountInfo},
    /// Macro para declarar el punto de entrada del programa
    entrypoint,
    /// Tipo de resultado devuelto por el punto de entrada del programa
    entrypoint::ProgramResult,
    ///macro para imprimir mensajes a la consola de la cadena de bloques
    msg,
    /// controlador de errores específicos del programa
    program_error::ProgramError,
    /// Gestión de claves
    pubkey::Pubkey,
};

/// Define el tipo de estado almacenado en las cuentas
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BuyAccount {
    /// número de acciones compradas
    pub counter: u32,
}

/// Declarar y exportar el punto de entrada del programa
entrypoint!(process_instruction);

/// Implementación del punto de entrada del programa
pub fn process_instruction(
    /// Clave pública de la cuenta en la que se cargó el programa de venta de acciones
    program_id: &Pubkey,
    /// La cuenta en la que se va a comprar las acciones
    accounts: &[AccountInfo],

    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Nuevo accion");
    /// Iterar las cuentas existente
    let accounts_iter = &mut accounts.iter();
    /// Obtener la cuenta en la que se va a comprar
    let account = next_account_info(accounts_iter)?;
    /// La cuenta debe ser propiedad del programa para poder modificar sus datos
    if account.owner != program_id {
        msg!("La cuenta de compra no tiene la identificación de programa correcta");
        return Err(ProgramError::IncorrectProgramId);
    }
    /// Incrementar y almacenar la cantidad de veces que se ha realizado compras en una la cuenta
    let mut buy_account = BuyAccount::try_from_slice(&account.data.borrow())?;
    buy_account.counter += 1;
    buy_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    msg!("¡Comprado {} vez(es)!", buy_account.counter);
    Ok(())
}

/// Configuracion de pruebas
/// este modulo solo se compila si se ejecutan las pruebas
#[cfg(test)]
///modulo de pruebas
mod test {
    ///Importa los modulos de la clase padre, (cadigo principal)
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;
    /// crea los modulos de prueba, asi como las variables y configuraciones necesarias
    #[test]
    fn test_sanity() {
        /// Preparar datos de prueba
        /// nueva instancia de clave publica
        let program_id = Pubkey::default();
        /// nueva instancia de clave publica
        let key = Pubkey::default();
        /// inicializacion de cantidad  solanas
        let mut lamports = 0;
        /// Lectura de creaciones
        let mut data = vec![0; mem::size_of::<u32>()];
        /// declara el dueño del prograna
        let owner = Pubkey::default();
        /// crea cuenta de prueba
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

        /// Crear un vector de cuentas para la prueba
        let accounts = vec![account];

        /// Asegurarse de que el contador de compras comienza en 0
        assert_eq!(
            BuyAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        /// Llamar al programa una vez
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        /// Verificar que el contador de compras se incrementa a 1
        assert_eq!(
            BuyAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );

        /// Llamar al programa nuevamente
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        /// Verificar que el contador de compras se incrementa a 2
        assert_eq!(
            BuyAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
