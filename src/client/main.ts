/**
 * Store books
 */
import {
  // Función para establecer una conexión a la red de Solana
  establishConnection,
  // Función para determinar quién paga las tarifas
  establishPayer,
  // Función para verificar si el programa está implementado
  checkProgram,
  // Función para realizar una operación de "vender libro"
  sellBook,
  // Función para obtener el número de compras realizadas
  reportBuys,
  // Importa las funciones desde el módulo './store_books'
} from './store_books';

async function main() {
  console.log("Solana account...");

  // Establece una conexión con el clúster de Solana
  await establishConnection();

  // Determina quién pagará las tarifas
  await establishPayer();

  // Verifica si el programa ha sido implementado
  await checkProgram();

  // Realiza una operación de "vender libro"
  await sellBook();

  // Obtiene la cantidad de compras realizadas
  await reportBuys();

  console.log('Éxito');
}

main().then(
    () => process.exit(),
    err => {
      console.error(err);
      process.exit(-1);
    },
);
