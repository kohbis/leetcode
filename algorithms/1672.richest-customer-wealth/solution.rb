# @param {Integer[][]} accounts
# @return {Integer}
def maximum_wealth(accounts)
  ## One-line solution ##
  # accounts.map {|a| a.sum }.max

  max = 0
  accounts.each do |a|
    sum = 0
    a.each {|n| sum += n }
    max = sum if sum > max
  end
  max
end
