#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::primitives::AccountId;

/// Interface
#[ink::trait_definition]
pub trait IFlipper {
    #[ink(message)]
    fn pause(&mut self) -> Result<(), ()>;
    #[ink(message)]
    fn is_paused(&self) -> bool;
    #[ink(message)]
    fn set_balance(&mut self, bal: u128) -> Result<(), ()>;
    #[ink(message)]
    fn get_balance(&mut self, addr: AccountId) -> Option<u128>;
}

/// Contract
#[ink::contract]
pub mod flipper {
    use super::IFlipper;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        is_paused: bool,
        owner_address: AccountId,
        balances: ink::storage::Mapping<AccountId, Balance>,
    }

    /// Events
    #[ink(event)]
    pub struct Flipped {
        #[ink(topic)]
        caller: AccountId,
        #[ink(topic)]
        value: bool,
    }

    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let caller = Self::env().caller();
            let balances = ink::storage::Mapping::default();
            Self::env().emit_event(Flipped {
                caller,
                value: init_value,
            });
            Self {
                is_paused: init_value,
                owner_address: caller,
                balances,
            }
        }

        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default_constructor() -> Self {
            Self::new(Default::default())
        }
    }

    impl IFlipper for Flipper {
        #[ink(message)]
        fn pause(&mut self) -> Result<(), ()> {
            self.is_paused = !self.is_paused;
            Ok(())
        }

        #[ink(message)]
        fn is_paused(&self) -> bool {
            self.is_paused
        }
        #[ink(message)]
        fn set_balance(&mut self, bal: u128) -> Result<(), ()> {
            let caller = Self::env().caller();
            self.balances.insert(caller, &bal);
            Ok(())
        }

        #[ink(message)]
        fn get_balance(&mut self, addr: AccountId) -> Option<u128> {
            return self.balances.get(addr);
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use ink::env::test::set_caller;

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default_constructor();
            assert_eq!(flipper.is_paused(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.is_paused(), false);
            flipper.pause().unwrap();
            assert_eq!(flipper.is_paused(), true);
        }
        #[ink::test]
        fn it_set_balance() {
            let mut flipper = Flipper::new(false);

            let alice = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            // Set Alice as the caller of the transaction
            set_caller::<Environment>(alice);

            flipper.set_balance(12).unwrap();

            // check balance
            assert_eq!(
                flipper.get_balance(alice),
                Some(12)
            );
        }
    }
}
