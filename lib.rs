#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;
    use trait_erc20::{TERC20, Error, Result};
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
    /// 和链上相关的都会自动引入
       total_supply: Balance,
       balances: Mapping<AccountId, Balance>,
       ///allowances（授权额度）是一个常见的概念，它与 Account 结构体和代币转账相关。
       /// 它用于记录一个账户通过授权允许其他账户可以从自己的账户中转移一定数量的代币。
       allowance: Mapping<(AccountId, AccountId), Balance>
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
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

  


    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        /// 执行且执行一次
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::new();
            ///cargo expand
            balances.insert(Self::env().caller(), &total_supply);

            Self::env().emit_event(
                Transfer {
                    from: None,
                    to: Some(Self::env().caller()),
                    value: total_supply
                }
            );

            Self { 
                total_supply,
                balances,
                ..Default::default()
            }
        }

       

        pub fn transfer_helper(&mut self, from: &AccountId, to: &AccountId, value: Balance) -> Result<()> {
            let balance_from = self.balance_of(*from);
            let balance_to = self.balance_of(*to);

            if value > balance_from {
                return Err(Error::BalanceTooLow);
            }
            self.balances.insert(from, &(balance_from - value));
            self.balances.insert(to, &(balance_to + value));

            self.env().emit_event(
                Transfer {
                    from: Some(*from),
                    to: Some(*to),
                    value
                }
            );

            Ok(())
        }
    }

        impl TERC20 for Erc20 {
            #[ink(message)] // 只读不可变
             fn total_supply(&self) -> Balance {
                self.total_supply
            }
            /// A message that can be called on instantiated contracts.
            /// This one flips the value of the stored `bool` from `true`
            /// to `false` and vice versa.
            #[ink(message)]
             fn balance_of(&self, who: AccountId) -> Balance {
                self.balances.get(&who).unwrap_or_default()
            }
            #[ink(message)]
             fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
                let sender  = self.env().caller();
                return self.transfer_helper(&sender, &to, value);
            }
    
            #[ink(message)]
             fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
                let sender = self.env().caller();
                let mut allowance = self.allowance.get(&(from, sender)).unwrap_or_default();
    
                if allowance < value {
                    return Err(Error::AllowanceTooLow);
                }
    
                self.allowance.insert(&(from, sender), &(allowance - value));
                return self.transfer_helper(&from, &to, value);
            }
    
            #[ink(message)]
             fn approve(&mut self, to: AccountId, value:Balance) -> Result<()> {
                let sender = self.env().caller();
                self.allowance.insert(&(sender, to), &value);
    
                self.env().emit_event(Approval {
                    from: sender,
                    to,
                    value
                });
    
            Ok(())
    
            }
        }
    
    

    #[cfg(test)]
    mod tests {

        use super::*;
        type Event =  <Erc20 as ink::reflect::ContractEventBase>::Type;
        #[ink::test]
        fn constructor_works() {
            let erc20 = Erc20::new(10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(erc20.total_supply(), 10000);
            assert_eq!(erc20.balance_of(accounts.alice), 10000);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();

            let event = &emitted_events[0];

            let decoded = <Event as scale::Decode>::decode(&mut &event.data[..]).expect("decoded error");

            match decoded {
                Event::Transfer(Transfer{from, to, value}) => {
                    assert_eq!(from.is_none(), true);
                    assert_eq!(to, Some(accounts.alice), "nint to error");
                    assert_eq!(value, 10000, "nint value error");
                },
                _ => panic!("match for error")
            }

        }   

        #[ink::test]
        fn transfer_should_work() {
            let mut erc20 = Erc20::new(10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let res = erc20.transfer(accounts.bob, 12);

            assert!(res.is_ok());
            //assert!(erc20.balance_of(accounts.alice), 10000_12);
            assert_eq!(erc20.balance_of(accounts.bob), 12);
        }

        #[cfg(feature = "e2e-tests")]
        mod e2e_tests{
            use super::*;
            type Event =  <Erc20 as ink::reflect::ContractEventBase>::Type;

            #[ink_e2e::test]
            async fn e2e_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                let total_supply = 123;
                let constructor = Erc20Ref::new(total_supply);
                let contract_acc_id = client.instantiated(
                    "erc20",
                    &ink_e2e,
                    constractor,
                    0,
                    None
                ).await
                .expect("instantiate faild")
                .account_id;

            let alice_acc  = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

            }
        
        }

    }


    
    
}
