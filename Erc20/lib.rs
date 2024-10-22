#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod Erc20 {

    use ink::storage::Mapping;
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Erc20 {
        name: String,
        owner: AccountId,
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    impl Erc20 {

        #[ink(constructor)]
        pub fn new(token_name: String, total_supply: Balance) -> Self {
            let default_balances = Mapping::default();
            let caller = Self::env().caller();

            Self { 
                name: token_name,
                owner: caller,
                total_supply,
                balances: default_balances,
                allowances: Default::default()
            }
        }

        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance 
        {
            self.total_supply
        }


        #[ink(message)]
        pub fn balance_of(&self, user: AccountId) -> Balance {
            self.balances.get(&user).unwrap_or_default()
        }

        
    }

    
    #[cfg(test)]
    mod tests {
       
        use super::*;

        #[ink::test]
        fn name_works() {
            let Erc20 = Erc20::new("Hivve".to_string(), 10000000);
            assert_eq!(Erc20.name(), "Hivve".to_string());
        }

        #[ink::test]
        fn total_supply_works() {
            let Erc20 = Erc20::new("Hivve".to_string(), 10000000);
            assert_eq!(Erc20.total_supply(), 10000000);
        }

        #[ink::test]
        fn balance_of_works() {
            let Erc20 = Erc20::new("Hivve".to_string(), 1000000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(Erc20.balance_of(accounts.alice),0);
        }
        
    }


}