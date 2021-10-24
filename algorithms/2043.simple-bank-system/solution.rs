struct Bank {
    balance: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Bank { balance: balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let len = self.balance.len();
        let a1 = account1 as usize;
        let a2 = account2 as usize;

        if len >= a1 && len >= a2 {
            if self.balance[a1 - 1] >= money {
                self.balance[a1 - 1] -= money;
                self.balance[a2 - 1] += money;
                return true;
            }
        }

        return false;
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let a = account as usize;

        if self.balance.len() >= a {
            self.balance[a - 1] += money;
            return true;
        }

        return false;
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let a = account as usize;

        if self.balance.len() >= a {
            if self.balance[a - 1] >= money {
                self.balance[a - 1] -= money;
                return true;
            }
        }

        return false;
    }
}

// /**
//  * Your Bank object will be instantiated and called as such:
//  * let obj = Bank::new(balance);
//  * let ret_1: bool = obj.transfer(account1, account2, money);
//  * let ret_2: bool = obj.deposit(account, money);
//  * let ret_3: bool = obj.withdraw(account, money);
//  */
