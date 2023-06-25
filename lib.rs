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

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BalanceTooLow,
        AllowanceTooLow,
    }

    type Result<T> = core::result::Result<T, Error>;

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

        #[ink(message)] // 只读不可变
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }
        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).unwrap_or_default()
        }
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let sender  = self.env().caller();
            return self.transfer_helper(&sender, &to, value);
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            let mut allowance = self.allowance.get(&(from, sender)).unwrap_or_default();

            if allowance < value {
                return Err(Error::AllowanceTooLow);
            }

            self.allowance.insert(&(from, sender), &(allowance - value));
            return self.transfer_helper(&from, &to, value);
        }

        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, value:Balance) -> Result<()> {
            let sender = self.env().caller();
            self.allowance.insert(&(sender, to), &value);

            self.env().emit_event(Approval {
                from: sender,
                to,
                value
            });

        Ok(())

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
                    from: *from,
                    to: *to,
                    value
                }
            );

            Ok(())
        }
    
    }

    #[cfg(test)]
    mod tests {
        use ink_e2e::subxt::ext::sp_runtime::serde::de::value;
        use scale::Decode;

        use super::*;
        type Event = <Erc20 as ink::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn constructor_works() {
            let erc20 = Erc20::new(10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(erc20.total_supply(), 10000);
            assert_eq!(erc20.balance_of(accounts.alice), 10000);

            let emited_events = ink::env::test::recorded_events().collect::<Vec<_>>();

            let event = &emited_events[0];

            let decoded = <Event as scale::Decode>::decode(&mut &event[..]).expect("decoded error");

            match decoded {
                Event::Transfer(Transfer(from, to, value)) => {
                    assert_eq!(from.is_none(), "nint for error");
                    assert_eq!(to, Some(accounts.alice), "nint to error");
                    
                }
            }

        }   

    }


    
    
}
