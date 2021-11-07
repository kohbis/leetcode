class Bank

=begin
    :type balance: Integer[]
=end
  def initialize(balance)
    @balance = balance
  end

=begin
    :type account1: Integer
    :type account2: Integer
    :type money: Integer
    :rtype: Boolean
=end
  def transfer(account1, account2, money)
    if @balance.size >= account1 && @balance.size >= account2
      if @balance[account1 - 1] >= money
        @balance[account1 - 1] -= money
        @balance[account2 - 1] += money

        return true
      end
    end

    false
  end

=begin
    :type account: Integer
    :type money: Integer
    :rtype: Boolean
=end
  def deposit(account, money)
    if @balance.size >= account
      @balance[account - 1] += money

      return true
    end

    false
  end

=begin
    :type account: Integer
    :type money: Integer
    :rtype: Boolean
=end
  def withdraw(account, money)
    if @balance.size >= account
      if @balance[account - 1] >= money
        @balance[account - 1] -= money

        return true
      end
    end

    false
  end
end

# Your Bank object will be instantiated and called as such:
# obj = Bank.new(balance)
# param_1 = obj.transfer(account1, account2, money)
# param_2 = obj.deposit(account, money)
# param_3 = obj.withdraw(account, money)
