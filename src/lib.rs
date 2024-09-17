use std::collections::HashSet;

// Find all our documentation at https://docs.near.org
use near_sdk::{env, log, near, store::IterableSet, Gas};

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    near1000: IterableSet<i32>,
    near10000: IterableSet<i32>,
    near100000: IterableSet<i32>,
    near1000000: IterableSet<i32>,

    native1000: HashSet<i32>,
    native10000: HashSet<i32>,
    native100000: HashSet<i32>,
    native1000000: HashSet<i32>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            near1000: IterableSet::new(b"1"),
            near10000: IterableSet::new(b"2"),
            near100000: IterableSet::new(b"3"),
            near1000000: IterableSet::new(b"4"),
            
            native1000: HashSet::new(),
            native10000: HashSet::new(),
            native100000: HashSet::new(),
            native1000000: HashSet::new(),
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {

    // #[init]
    // pub fn new() -> Self {
    //     let mut initial = Contract {
    //         near1000: IterableSet::new(b"1"),
    //         near10000: IterableSet::new(b"2"),
    //         near100000: IterableSet::new(b"3"),
    //         near1000000: IterableSet::new(b"4"),
            
    //         native1000: HashSet::new(),
    //         native10000: HashSet::new(),
    //         native100000: HashSet::new(),
    //         native1000000: HashSet::new(),
    //     };

    //     for value in 1..=1_000_000 {
    //          if value < 1000
    //          {
    //             initial.near1000.insert(value);
    //             initial.native1000.insert(value);
    //          }

    //          if value < 10000 {
    //             initial.near10000.insert(value);
    //             initial.native10000.insert(value);
    //          }
             
    //          if value < 100000 {
    //             initial.near100000.insert(value);
    //             initial.native100000.insert(value);
    //          }

    //          if value < 1000000 {
    //             initial.near1000000.insert(value);
    //             initial.native1000000.insert(value);
    //          }
    //     }

    //     return initial;
    // }


    pub fn fill_near_1000(&mut self) -> i32 {
        let max = self.near1000.len() as i32;

        for value in max..=1_000 {
            self.near1000.insert(value);

            if env::used_gas() > Gas::from_tgas(200)
            {
                return value;
            }
        }

        return 1000;
    }

    pub fn fill_native_1000(&mut self) {
        for value in 1..=1_000 {
            self.native1000.insert(value);
        }
    }

    pub fn fill_near_10_000(&mut self) -> i32 {
        let max = self.near10000.len() as i32;

        log!("max {}", max);

        for value in max..=10_000 {
            self.near10000.insert(value);

            let used_gas = env::used_gas();

            if value % 100 == 0 {
                log!("used gas {}", used_gas);
            }

            if used_gas > Gas::from_tgas(50)
            {
                return value;
            }
        }

        return 10000;
    }

    pub fn fill_native_10_000(&mut self) {
        for value in 1..=10_000 {
            self.native10000.insert(value);
        }
    }

    pub fn check_near_1000(&self, value: i32) {
        self.near1000.contains(&value);
    }

    pub fn check_native_1000(&self, value: i32) {
        self.native1000.contains(&value);
    }

    pub fn check_near_10_000(&self) {
        log!("check_near_10_000 actual length {}", self.near10000.len());

        let value = 5000;
        //for value in 8000..=8010 {
            let result = self.near10000.contains(&value);
            log!("result {} {}", result, value);
        //}
    }

    pub fn check_native_10_000(&self) {
        log!("check_near_10_000 actual length {}", self.native10000.len());

        let value = 5000;
        //for value in 8000..=8010 {
            let result = self.native10000.contains(&value);
            log!("result {} {}", result, value);
        //}
    }

    pub fn add_near_1000(&mut self, value: i32) {

        let start_timestamp = env::block_timestamp();
        self.near1000.insert(value);
        let end_timestamp = env::block_timestamp();

        let duration_ns = end_timestamp - start_timestamp;

        log!("add_near_1000 took {} nanoseconds", duration_ns);
    }

    pub fn add_native_1000(&mut self, value: i32) {
        let start_timestamp = env::block_timestamp();
        self.native1000.insert(value);
        let end_timestamp = env::block_timestamp();

        let duration_ns = end_timestamp - start_timestamp;

        log!("add_native_1000 took {} nanoseconds", duration_ns);
    }
}