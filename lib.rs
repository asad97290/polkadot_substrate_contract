#![cfg_attr(not(feature = "std"), no_std, no_main)]



/// Interface
#[ink::trait_definition]
pub trait IFlipper {
    #[ink(message)]
    fn flip(&mut self) -> Result<(),()>;
    #[ink(message)]
    fn get(&self) -> bool;
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
        value: bool,
        address:AccountId,
        balances: ink::storage::Mapping<AccountId, Balance>,
    }

    /// Events
    #[ink(event)]
    pub struct Flipped {
        #[ink(topic)]
        caller: AccountId,
        #[ink(topic)]
        value:bool
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
            Self { value: init_value,address: caller,balances}
        }

        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default_constructor() -> Self {
            Self::new(Default::default())
        }

        
    }

    impl IFlipper for Flipper{
        #[ink(message)]
        fn flip(&mut self) -> Result<(),()>{
            self.value = !self.value;
            Ok(())
        }

        #[ink(message)]
         fn get(&self) -> bool {
            self.value
        }
    }
   
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default_constructor();
            assert_eq!(flipper.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip().unwrap();
            assert_eq!(flipper.get(), true);
        }
    }


}
