# @param {Integer[]} candies
# @param {Integer} extra_candies
# @return {Boolean[]}
def kids_with_candies(candies, extra_candies)
  max_candies = candies.sort[-1]
  candies.map { |candy| candy + extra_candies >= max_candies }
end
