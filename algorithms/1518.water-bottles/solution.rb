# @param {Integer} num_bottles
# @param {Integer} num_exchange
# @return {Integer}
def num_water_bottles(num_bottles, num_exchange)
  count = 0
  full, empty = num_bottles, 0

  while 0 < full
    count += full
    empty += full
    full = empty / num_exchange
    empty = empty % num_exchange
  end

  count
end
