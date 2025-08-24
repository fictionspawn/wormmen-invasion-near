// Find all our documentation at https://docs.near.org
use near_sdk::{log, near};

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    wormman_state: bool,
    character_coords_x: i32,
    character_coords_y: i32,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            wormman_state: true,
            character_coords_x: 0,
            character_coords_y: 0,
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {

    pub fn move_character_left(&mut self) {
        self.character_coords_x -= 1;
        log!("Moved character one step left. Coordinate: {}", self.character_coords_x);
        if self.character_coords_x == 8 {
            log!("There's a dead body on the ground.");
        }
        if self.character_coords_x == 5 {
            log!("A ladder is going up.");
        }
    }
    
    pub fn move_character_up(&mut self) {
        if character_coords_y == 0 && character_coords_x == 5 {
            character_coords_y == 1;
            log!("Climbed up the ladder. Coordinate: {}, {}", self.character_coords_x, self.character_coords_y,)
        }
    }

    pub fn move_character_right(&mut self) {
        self.character_coords_x += 1;
        log!("Moved character one step right. Coordinate: {}", self.character_coords_x);
        if self.character_coords_x == 8 {
            log!("There's a dead body on the ground.");
        }
        if self.character_coords_x == 5 {
            log!("A ladder is going up.");
        }
    }

    pub fn get_character_coords_x(&self) -> i32 {
        self.character_coords_x
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_wormman_state(&self) -> bool {
        self.wormman_state
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn kill_wormman(&mut self) {
        log!("Killing wormman.");
        self.wormman_state = false;
    }

    pub fn revive_wormman(&mut self) {
        log!("Reviving wormman.");
        self.wormman_state = true;
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
