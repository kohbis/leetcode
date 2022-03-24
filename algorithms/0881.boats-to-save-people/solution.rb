# @param {Integer[]} people
# @param {Integer} limit
# @return {Integer}
def num_rescue_boats(people, limit)
  people.sort!

  left, right = 0, people.size - 1
  count = 0

  while left < right
    if people[left] + people[right] <= limit
      left += 1
    end
    right -= 1
    count += 1
  end

  count += 1 if left == right
  count
end
