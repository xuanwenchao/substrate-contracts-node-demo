#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        /// Stores a single `bool` value on the storage.
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error{
        BalanceTooLow,
        AllowancesTooLow,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
            #[ink(constructor)]
            pub fn new(total_supply: Balance) -> Self {
                let mut balances = Mapping::new();
                balances.insert(Self::env().caller(), &total_supply);
                Self {
                    total_supply,
                    balances,
                    ..Default::default()
                }
            }

            #[ink(constructor)]
            pub fn default() -> Self {
                Self::new(Default::default())
            }

            #[ink(message)]
            pub fn total_supply(&self) -> Balance {
                self.total_supply
            }

            #[ink(message)]
            pub fn balance_of(&self, who: AccountId) -> Balance {
                self.balances.get(&who).unwrap_or_default()
            }

            #[ink(message)]
            pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
                let from = self.env().caller();  // get the caller 
                self.transfer_helper(&from, &to, value)
            }


            #[ink(message)]
            pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
                let sender = self.env().caller();  // get the caller 
                self.allowances.insert(&(sender, spender), &value);
                self.env().emit_event(Approval{from:sender,to:spender,value});
                Ok(())
            }

            #[ink(message)]
            pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
                let sender = self.env().caller();
                let allowances = self.allowances.get(&(from, sender)).unwrap_or_default();
                if allowances < value {
                    return Err(Error::AllowancesTooLow);
                }

                self.allowances.insert(&(from, sender), &(allowances - value));

                self.transfer_helper(&from, &to, value)
            }

            pub fn transfer_helper(&mut self, from: &AccountId, to: &AccountId, value: Balance) -> Result<()> {
                let balance_from = self.balance_of(*from);
                let balance_to = self.balance_of(*to);

                if value > balance_from {
                    return Err(Error::BalanceTooLow);
                }

                self.balances.insert(from, &(balance_from - value));
                self.balances.insert(to, &(balance_to + value));
                
                self.env().emit_event(Transfer {
                    from:*from,
                    to:*to,
                    value,
                });

                Ok(())
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
            let erc20 = Erc20::default();
            assert_eq!(erc20.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut erc20 = Erc20::new(false);
            assert_eq!(erc20.get(), false);
            erc20.flip();
            assert_eq!(erc20.get(), true);
        }
    }


    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = Erc20Ref::default();

            // When
            let contract_account_id = client
                .instantiate("erc20", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<Erc20Ref>(contract_account_id.clone())
                .call(|erc20| erc20.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = Erc20Ref::new(false);
            let contract_account_id = client
                .instantiate("erc20", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<Erc20Ref>(contract_account_id.clone())
                .call(|erc20| erc20.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<Erc20Ref>(contract_account_id.clone())
                .call(|erc20| erc20.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<Erc20Ref>(contract_account_id.clone())
                .call(|erc20| erc20.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
