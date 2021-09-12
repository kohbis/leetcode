# @param {Integer[]} stones
# @return {Integer}
def last_stone_weight(stones)
  loop do
    return 0 if stones.length == 0
    return stones[0] if stones.length == 1

    stones.sort!
    l, m = stones.pop, stones.pop
    diff = l - m
    stones.push(diff) unless diff == 0
  end
end
