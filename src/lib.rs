// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, store::LookupMap, env, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
//use crate::borsh::BorshSchema;
use near_sdk_macros::NearSchema;
//use std::collections::HashMap;
//pub use borsh_derive::BorshSchema;
//use unstable__schema::BorshSchema;

// Define the contract structure
#[near(contract_state)]
pub struct Contract { 
    player_states: LookupMap<AccountId, PlayerState>,
   // game_state: GameState,
}

#[derive(BorshDeserialize, BorshSerialize, NearSchema, Default, Clone)]
#[abi(borsh)]
pub struct PlayerState {
    wormman_dead: bool,
    character_coords_x: i32,
    character_coords_y: i32,
}
/*
#[derive(BorshDeserialize, BorshSerialize, NearSchema, Default, Clone)]
pub struct GameState {
#[abi(borsh)]
    wormman_dead: bool,
    door_open: bool,
}
*/
// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            player_states: LookupMap::new(b"player_states".to_vec()),
        }
    }
}


// Implement the contract structure
#[near]
impl Contract {
    fn get_default_state() -> PlayerState {
        PlayerState {
            wormman_dead: false,
            character_coords_x: 0,
            character_coords_y: 0,
        }
    }

    fn get_player_state(&mut self) -> PlayerState {
        let account_id = env::signer_account_id();

        match self.player_states.get(&account_id) {
            Some(state) => state.clone(),
            None => {
                let default_state = Self::get_default_state();
                self.player_states.insert(account_id, default_state.clone());
                default_state
            }
        }
    }
 /*   fn get_player_state(&self, account_id: AccountId) -> PlayerState {

       // let account_id = env::signer_account_id();
        self.player_states.get(&account_id).unwrap().clone()
    }*/

    fn set_player_state(&mut self, state: PlayerState) {
        let account_id = env::signer_account_id();
        self.player_states.insert(account_id, state);
    }

    pub fn move_character_left(&mut self) {
        let mut state = self.get_player_state();
        state.character_coords_x -= 1;
        log!("Moved character one step left. Coordinates: {}, {}", state.character_coords_x, state.character_coords_y);
        if state.character_coords_x == 8 && state.character_coords_y == 0 {
            log!("There's a dead body on the ground.");
        }
        if state.character_coords_x == 5 && state.character_coords_y == 0 {
            log!("A ladder is going up.");
        }
        self.set_player_state(state);
    }
   
    pub fn move_character_right(&mut self) {
        let mut state = self.get_player_state();
        state.character_coords_x += 1;
        log!("Moved character one step right. Coordinate: {}, {}", state.character_coords_x, state.character_coords_y);
        if state.character_coords_x == 8 && state.character_coords_y == 0 {
            log!("There's a dead body on the ground.");
        }
        if state.character_coords_x == 5 && state.character_coords_y == 0 {
            log!("A ladder is going up.");
        }
        self.set_player_state(state);
    }
 
    pub fn move_character_up(&mut self) {
        let mut state = self.get_player_state();
        if state.character_coords_y == 0 && state.character_coords_x == 5 {
            state.character_coords_y = 1;
            log!("Climbed up the ladder. Coordinate: {}, {}", state.character_coords_x, state.character_coords_y);
            self.set_player_state(state);
        }
    }

    pub fn move_character_down(&mut self) {
        let mut state = self.get_player_state();
        if state.character_coords_y == 1 && state.character_coords_x == 5 {
            state.character_coords_y = 0;
            log!("Climbed down the ladder. Coordinates: {}, {}", state.character_coords_x, state.character_coords_y);
            self.set_player_state(state);
        }
    }

    pub fn get_character_coords_x(&mut self) -> i32 {
        let state = self.get_player_state();
        state.character_coords_x
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_wormman_state(&mut self) -> bool {
        let state = self.get_player_state();
        state.wormman_dead
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn kill_wormman(&mut self) {
        let mut state = self.get_player_state();
        log!("Killing wormman.");
        state.wormman_dead = true;
        self.set_player_state(state);
    }

    pub fn revive_wormman(&mut self) {
        let mut state = self.get_player_state();
        log!("Reviving wormman.");
        state.wormman_dead = false;
        self.set_player_state(state);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_wormman_state() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_wormman_state(), true);
    }

    #[test]
    fn kill_then_get_wormman_state() {
        let mut contract = Contract::default();
        contract.kill_wormman();
        assert_eq!(contract.get_wormman_state(), false);
    }
    
    #[test]
    fn revive_then_get_wormman_state() {
        let mut contract = Contract::default();
        contract.revive_wormman();
        assert_eq!(contract.get_wormman_state(), true);
    }
}
