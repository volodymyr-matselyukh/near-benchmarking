use std::collections::HashSet;

// Find all our documentation at https://docs.near.org
use near_sdk::{env, log, near, store::{IterableSet, LookupMap}, Gas};

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    near100: LookupMap<i32, IterableSet<i32>>,
    near500: LookupMap<i32, IterableSet<i32>>,
    near1000: LookupMap<i32, IterableSet<i32>>,
    near10000: LookupMap<i32, IterableSet<i32>>,
    near100000: LookupMap<i32, IterableSet<i32>>,
    near1000000: LookupMap<i32, IterableSet<i32>>,

    native100: LookupMap<i32, HashSet<i32>>,
    native500: LookupMap<i32, HashSet<i32>>,
    native1000: LookupMap<i32, HashSet<i32>>,
    native10000: LookupMap<i32, HashSet<i32>>,
    native100000: LookupMap<i32, HashSet<i32>>,
    native1000000: LookupMap<i32, HashSet<i32>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            near100: LookupMap::new(b"o"),
            near500: LookupMap::new(b"0"),
            near1000: LookupMap::new(b"1"),
            near10000: LookupMap::new(b"2"), 
            near100000: LookupMap::new(b"3"), 
            near1000000: LookupMap::new(b"4"),
            
            native100: LookupMap::new(b"l"),
            native500: LookupMap::new(b"9"),
            native1000: LookupMap::new(b"5"),
            native10000: LookupMap::new(b"6"),
            native100000: LookupMap::new(b"7"), 
            native1000000: LookupMap::new(b"8"),
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {

    // 100
    pub fn fill_near_100(&mut self) -> i32 {
        let max = 100;

        let existing_set_unwrapped = self.near100.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            return Contract::fill_near_collection_in_loop(existing_set, max);
        }

        let mut new_set = IterableSet::new(b"b");

        let index = Contract::fill_near_collection_in_loop(&mut new_set, max);

        self.near100.insert(0, new_set);

        return index;
    }

    pub fn fill_native_100(&mut self) -> i32 {
        let max = 100;

        let existing_set_unwrapped = self.native100.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            return Contract::fill_native_collection(existing_set, max);
        }

        let mut new_set = HashSet::new();

        let index = Contract::fill_native_collection(&mut new_set, max);

        self.native100.insert(0, new_set);

        return index;
    }
    
    // 500
    pub fn fill_near_500(&mut self) -> i32 {
        let max = 500;

        let existing_set_unwrapped = self.near500.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            return Contract::fill_near_collection_in_loop(existing_set, max);
        }

        let mut new_set = IterableSet::new(b"b");

        let index = Contract::fill_near_collection_in_loop(&mut new_set, max);

        self.near500.insert(0, new_set);

        return index;
    }

    pub fn fill_native_500(&mut self) -> i32 {
        let max = 500;

        let existing_set_unwrapped = self.native500.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            return Contract::fill_native_collection(existing_set, max);
        }

        let mut new_set = HashSet::new();

        let index = Contract::fill_native_collection(&mut new_set, max);

        self.native500.insert(0, new_set);

        return index;
    }

    // 1k

    pub fn fill_near_1000(&mut self) -> i32 {
        let max = 1000;

        let existing_set_unwrapped = self.near1000.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            return Contract::fill_near_collection_in_loop(existing_set, max);
        }

        let mut new_set = IterableSet::new(b"b");

        let index = Contract::fill_near_collection_in_loop(&mut new_set, max);

        self.near1000.insert(0, new_set);

        return index;
    }

    pub fn fill_native_1000(&mut self) -> i32 {
        let max = 1000;

        let existing_set_unwrapped = self.native1000.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            return Contract::fill_native_collection(existing_set, max);
        }

        let mut new_set = HashSet::new();

        let index = Contract::fill_native_collection(&mut new_set, max);

        self.native1000.insert(0, new_set);

        return index;
    }

    // 10k

    pub fn fill_near_10_000(&mut self) -> i32 {
        let max = 10_000;

        let existing_set_unwrapped = self.near10000.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            return Contract::fill_near_collection_in_loop(existing_set, max);
        }

        let mut new_set = IterableSet::new(b"c");

        let index = Contract::fill_near_collection_in_loop(&mut new_set, max);

        self.near10000.insert(0, new_set);

        return index;
    }

    pub fn fill_native_10_000(&mut self) -> i32 {
        let max = 10_000;

        let existing_set_unwrapped = self.native10000.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            return Contract::fill_native_collection(existing_set, max);
        }

        let mut new_set = HashSet::new();

        let index = Contract::fill_native_collection(&mut new_set, max);

        self.native10000.insert(0, new_set);

        return index;
    }

    // 100k

    pub fn fill_near_100_000(&mut self) -> i32 {
        let max = 100_000;

        let existing_set_unwrapped = self.near100000.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            return Contract::fill_near_collection_in_loop(existing_set, max);
        }

        let mut new_set = IterableSet::new(b"d");

        let index = Contract::fill_near_collection_in_loop(&mut new_set, max);

        self.near100000.insert(0, new_set);

        return index;
    }

    pub fn fill_native_100_000(&mut self) -> i32 {
        let max = 100_000;

        let existing_set_unwrapped = self.native100000.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            return Contract::fill_native_collection(existing_set, max);
        }

        let mut new_set = HashSet::new();

        let index = Contract::fill_native_collection(&mut new_set, max);

        self.native100000.insert(0, new_set);

        return index;
    }
    
    // Checks 

    // check 100
    pub fn check_near_100(&self, value: i32) -> bool {
        let existing_set = self.near100.get(&0).unwrap();

        return existing_set.contains(&value);
    }

    pub fn check_native_100(&self, value: i32) -> bool {
        let existing_set = self.native100.get(&0).unwrap();

        return existing_set.contains(&value);
    }

    // check 500
    pub fn check_near_500(&self, value: i32) -> bool {
        let existing_set = self.near500.get(&0).unwrap();

        return existing_set.contains(&value);
    }

    pub fn check_native_500(&self, value: i32) -> bool {
        let existing_set = self.native500.get(&0).unwrap();

        return existing_set.contains(&value);
    }

    // check 1k
    pub fn check_near_1000(&self, value: i32) -> bool {
        let existing_set = self.near1000.get(&0).unwrap();

        return existing_set.contains(&value);
    }

    pub fn check_native_1000(&self, value: i32) -> bool {
        let existing_set = self.native1000.get(&0).unwrap();

        return existing_set.contains(&value);
    }

    // check 10k
    pub fn check_near_10_000(&self) {
        let existing_set = self.near10000.get(&0).unwrap();
        log!("check_near_10_000 actual length {}", existing_set.len());

        let value = 5000;
        
        let result = existing_set.contains(&value);
        log!("result {} {}", result, value);
    }

    pub fn check_native_10_000(&self) {
        let existing_set = self.native10000.get(&0).unwrap();
        log!("check_near_10_000 actual length {}", existing_set.len());

        let value = 5000;
        
        let result = existing_set.contains(&value);
        log!("result {} {}", result, value);
    }

    // check 100k
    pub fn check_near_100_000(&self) {
        let existing_set = self.near100000.get(&0).unwrap();
        log!("check_near_10_000 actual length {}", existing_set.len());

        let value = 5000;
        
        let result = existing_set.contains(&value);
        log!("result {} {}", result, value);
    }

    pub fn check_native_100_000(&self) {
        let existing_set = self.native100000.get(&0).unwrap();
        log!("check_near_10_000 actual length {}", existing_set.len());

        let value = 5000;
        
        let result = existing_set.contains(&value);
        log!("result {} {}", result, value);
    }

    pub fn check_near_multiple_100_000(&self) {
        let existing_set = self.near100000.get(&0).unwrap();
        log!("check_near_100_000 actual length {}", existing_set.len());

        for value in 8000..=8010 {
            let result = existing_set.contains(&value);
            log!("result {} {}", result, value);
        }
    }

    pub fn check_native_multiple_100_000(&self) {
        let existing_set = self.native100000.get(&0).unwrap();
        log!("check_near_100_000 actual length {}", existing_set.len());

        for value in 8000..=8010 {
            let result = existing_set.contains(&value);
            log!("result {} {}", result, value);
        }
    }

    // insert

    pub fn add_near_100_000(&mut self, value: i32) {
        let existing_set_unwrapped = self.near100000.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();
            
            existing_set.insert(value);

            return;
        }

        let mut new_set = IterableSet::new(b"d");

        new_set.insert(value);

        self.near100000.insert(0, new_set);
    }

    pub fn add_native_100_000(&mut self, value: i32) {
        let existing_set_unwrapped = self.native100000.get_mut(&0);
        
        if existing_set_unwrapped.is_some() {
            let existing_set = existing_set_unwrapped.unwrap();

            existing_set.insert(value);

            return;
        }

        let mut new_set = HashSet::new();

        new_set.insert(value);

        self.native100000.insert(0, new_set);
    }

    // static 
    fn fill_near_collection_in_loop(collection: &mut IterableSet<i32>, collection_max: i32) -> i32 {
        let max = collection.len() as i32;

        for value in max..=collection_max - 1 {
            collection.insert(value);

            let used_gas = env::used_gas();

            if value % 100 == 0 {
                log!("used gas {}", used_gas);
            }

            if used_gas > Gas::from_tgas(60)
            {
                return value;
            }
        }

        return collection_max;
    }

    fn fill_native_collection(collection: &mut HashSet<i32>, collection_max: i32) -> i32 {
        for value in 0..=collection_max - 1 {
            collection.insert(value);
        }

        return collection_max;
    }
}