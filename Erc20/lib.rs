#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {

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

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance
    }

    impl Erc20 {

        #[ink(constructor)]
        pub fn new(token_name: String, total_supply: Balance) -> Self {
            let mut default_balances = Mapping::default();
            let caller = Self::env().caller();

            default_balances.insert(caller, &total_supply);

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
            self.balances.get(user).unwrap_or_default()
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((&owner, &spender)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            let from = self.env().caller();

            self.transfer_from_to(&from, &to, value)
        }

        fn transfer_from_to(&mut self, from: &AccountId, to: &AccountId, value: Balance) -> Result<(), Error> {
            let from_balance = self.balance_of(*from);
            if from_balance <  value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance.checked_sub(value).unwrap()));

            let to_balance = self.balance_of(*to);

            self.balances.insert(to, &(to_balance.checked_add(value).unwrap()));

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }

    }

    
    #[cfg(test)]
    mod tests {
       
        use super::*;

        fn default_accounts() -> ink::env::test::DefaultAccounts<Environment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn alice() -> AccountId {
            default_accounts().alice
        }

        fn bob() -> AccountId {
            default_accounts().bob
        }


        #[ink::test]
        fn name_works() {
            let erc20 = Erc20::new("Hivve".to_string(), 10000000);
            assert_eq!(erc20.name(), "Hivve".to_string());
        }

        #[ink::test]
        fn total_supply_works() {
            let erc20 = Erc20::new("Hivve".to_string(), 10000000);
            assert_eq!(erc20.total_supply(), 10000000);
        }

        #[ink::test]
        fn balance_of_works() {
            let erc20 = Erc20::new("Hivve".to_string(), 1000000);
            assert_eq!(erc20.balance_of(alice()),1000000);
        }

        #[ink::test]
        fn transfer_works() {
            let mut erc20 = Erc20::new("Hivve".to_string(), 100000);
            
            assert!(erc20.transfer(bob(), 100).is_ok());
            assert_eq!(erc20.balance_of(bob()), 100);
            assert_eq!(erc20.balance_of(alice()), 100000 - 100);
        }
        
    }


}