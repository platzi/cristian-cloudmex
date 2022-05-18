use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ near_bindgen, setup_alloc};

setup_alloc!();

//1. estructura,  state del contrato y inicialización de los valores por default
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleMemeMuseum {

}

//inicializamos el state del contrato
impl Default for SimpleMemeMuseum {
    fn default() -> Self {
        Self {

       }
    }
}

