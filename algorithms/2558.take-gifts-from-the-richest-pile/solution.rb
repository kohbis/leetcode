# @param {Integer[]} gifts
# @param {Integer} k
# @return {Integer}
def pick_gifts(gifts, k)
  k.times do
    n = gifts.max
    gifts[gifts.index(n)] = Math.sqrt(n).to_i
  end
  gifts.sum
end
