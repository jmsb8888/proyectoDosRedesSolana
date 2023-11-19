/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-return */

import os from 'os'; // Importa el módulo 'os' para trabajar con operaciones del sistema operativo.
import fs from 'mz/fs'; // Importa el módulo 'fs' para trabajar con el sistema de archivos.
import path from 'path'; // Importa el módulo 'path' para trabajar con rutas de archivos y directorios.
import yaml from 'yaml'; // Importa el módulo 'yaml' para analizar archivos YAML.
import {Keypair} from '@solana/web3.js'; // Importa la clase 'Keypair' de la biblioteca '@solana/web3.js'.


async function getConfig(): Promise<any> {
  // Ruta al archivo de configuración del Solana CLI
  const CONFIG_FILE_PATH = path.resolve(
      os.homedir(),
      '.config',
      'solana',
      'cli',
      'config.yml',
  );

  // Lee el contenido del archivo de configuración y lo analiza como YAML
  const configYml = await fs.readFile(CONFIG_FILE_PATH, {encoding: 'utf8'});
  return yaml.parse(configYml); // Devuelve la configuración analizada
}

/**
 * Carga y analiza el archivo de configuración del Solana CLI para determinar qué URL RPC utilizar
 */
export async function getRpcUrl(): Promise<string> {
  try {
    const config = await getConfig();
    if (!config.json_rpc_url) throw new Error('URL RPC faltante');
    // Devuelve la URL RPC desde la configuración
    return config.json_rpc_url;
  } catch (err) {
    console.warn(
        'No se pudo leer la URL RPC desde el archivo de configuración del CLI de Solana, usando localhost por defecto',
    );
    // Si falla, usa una URL RPC local por defecto
    return 'http://127.0.0.1:8899';
  }
}

/**
 * Carga y analiza el archivo de configuración del Solana CLI para determinar qué clave de pago utilizar
 */
export async function getPayer(): Promise<Keypair> {
  try {
    const config = await getConfig();
    if (!config.keypair_path) throw new Error('Ruta de clave faltante');
    // Devuelve la clave de pago desde la configuración
    return await createKeypairFromFile(config.keypair_path);
  } catch (err) {
    console.warn(
        'No se pudo crear la clave de pago desde el archivo de configuración del CLI de Solana, generando una nueva clave aleatoria por defecto',
    );
    // Si falla, genera una nueva clave de pago aleatoria
    return Keypair.generate();
  }
}

/**
 * Crea un Keypair a partir de una clave secreta almacenada en un archivo como una matriz de bytes
 */
export async function createKeypairFromFile(
    filePath: string,
): Promise<Keypair> {
  const secretKeyString = await fs.readFile(filePath, {encoding: 'utf8'});
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  // Crea un Keypair a partir de la clave secreta del archivo
  return Keypair.fromSecretKey(secretKey);
}
