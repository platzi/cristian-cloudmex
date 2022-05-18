use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

// **************** Estructura de Meme *********************************
//
// 2.  Clases necesarias para el contrato
// Implementamos serde cuando necesitamos retornar la estructura serializada a JSON util en NEAR
// CLI y frotend. En este caso se utilizan ambas serializaciones ya que sera utilizado borsh en la
// serialización y deserialización del contrato en la blockchain de NEAR
//
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Meme {
    pub id: u64,
    pub creado_por: String,
    pub titulo: String,
    pub museo: String,
    pub url: String,
    pub donaciones: u128,
}

// Implemetación del trait Default para inicializar la estructura de Meme 
impl Default for Meme {
    fn default() -> Self {
        Meme {
            id: 0,
            creado_por: String::from(""),
            titulo: String::from(""),
            museo: String::from(""),
            url: String::from(""),
            donaciones: 0,
        }
    }
}

// Implementación del metodo new que permitira crear nuevos memes 
impl Meme {
    pub fn new(titulo: String, url: String, museo: String) -> Self {
        Self {
            //id: 0,
            //creado_por: String::from(""),
            id: env::block_index(),
            creado_por: env::signer_account_id(),
            titulo,
            museo,
            url,
            donaciones: 0,
        }
    }
}

//1. estructura,  state del contrato y inicialización de los valores por default
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleMemeMuseum {
    //Guardamos solo los ID para evitar tener que editar en ambos lugares cuando se modifique un meme.
    museos: UnorderedMap<String, Vec<u64>>,
    memes: UnorderedMap<u64, Meme>,
}

//inicializamos el state del contrato
impl Default for SimpleMemeMuseum {
    fn default() -> Self {
        Self {
            //inicializamos las colecciones 
            museos: UnorderedMap::new(b"u".to_vec()),
            memes: UnorderedMap::new(b"e".to_vec()),
        }
    }
}

