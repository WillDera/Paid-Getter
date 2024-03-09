#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod greeter {
    use ink::prelude::string::String;

    #[ink(event)]
    pub struct Greeted {
        from: Option<AccountId>,
        message: String,
    }

    #[ink(event)]
    pub struct Locked {
        message: String,
    }

    #[ink(storage)]
    pub struct Greeter {
        message: String,
        locked: bool,
        count: i32,
    }

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    // #[derive(Debug, PartialEq, Eq)]
    // #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Returned if caller has insufficient balance
        InsufficientBalance,
        /// Returned if transfer failed
        TransferFailed,
    }

    /// Type alias for the contract's `Result` type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Greeter {
        /// Creates a new greeter contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(message: String, locked: bool, count: i32) -> Self {
            Self {
                message,
                locked,
                count,
            }
        }

        /// Creates a new greeter contract initialized to 'Hello ink!'.
        #[ink(constructor)]
        pub fn default() -> Self {
            let default_message = String::from("Hello ink!");
            let default_lock = false;
            let default_count = 0;
            Self::new(default_message, default_lock, default_count)
        }

        #[ink(message)]
        pub fn get_balance(&self) -> u128 {
            self.env().balance()
        }

        /// Returns the current value of `message`.
        #[ink(message)]
        pub fn greet(&self) -> String {
            self.message.clone()
        }

        /// Sets `message` to the given value.
        #[ink(message, payable)]
        pub fn set_message(&mut self, new_message: String, value: Balance) -> Result<()> {
            if value < 100 {
                return Err(Error::InsufficientBalance);
            }

            ink::env::debug_println!("received payment: {}", self.env().transferred_value());

            match self.locked {
                false => {
                    if self.count > 1 && self.count % 5 == 0 {
                        self.count += 1;
                        self.message = String::from("You won!");
                        self.locked = true;

                        // withdraw everything in the contract to the caller
                        self.withdraw_all()?;
                    } else {
                        self.count += 1;
                        self.message = new_message;
                    }
                }
                true => self.env().emit_event(Locked {
                    message: String::from("Contract is Locked!"),
                }),
            }

            Ok(())
        }

        #[ink(message)]
        pub fn get_lock_state(&self) -> bool {
            self.locked
        }

        #[ink(message, payable)]
        pub fn unlock(&mut self, value: Balance) -> Result<()> {
            if !self.locked {
                self.env().emit_event(Locked {
                    message: String::from("Contract is Not Locked!"),
                })
            }
            if value < 1000 {
                return Err(Error::InsufficientBalance);
            }

            self.locked = false;

            ink::env::debug_println!("received payment: {}", self.env().transferred_value());

            Ok(())
        }

        fn withdraw_all(&mut self) -> Result<()> {
            if self.locked {
                let balance = self.env().balance();
                self.env()
                    .transfer(self.env().caller(), balance)
                    .map_err(|_| Error::TransferFailed)?;
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::env::pay_with_call;

        use super::*;

        #[ink::test]
        fn new_works() {
            let message = "Hello ink! v4".to_string();
            let count = 0;
            let locked = false;
            let greeter = Greeter::new(message.clone(), locked, count);
            assert_eq!(greeter.greet(), message);
        }

        #[ink::test]
        fn default_new_works() {
            let greeter = Greeter::default();
            let default_message = String::from("Hello ink!");
            assert_eq!(greeter.greet(), default_message);
        }

        #[ink::test]
        fn set_message_works() {
            let message_1 = String::from("gm ink!");
            let locked_1 = false;
            let count_1 = 0;
            let mut greeter = Greeter::new(message_1.clone(), locked_1, count_1);
            assert_eq!(greeter.greet(), message_1);
            let message_2 = String::from("gn");

            let initial_balance = greeter.get_balance();
            pay_with_call!(greeter.set_message(message_2.clone(), 100), 100);
            assert_eq!(greeter.get_balance(), initial_balance + 100);
            assert_eq!(greeter.greet(), message_2);
        }
    }
}
