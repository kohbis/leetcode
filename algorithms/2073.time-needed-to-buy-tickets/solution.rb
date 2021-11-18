# @param {Integer[]} tickets
# @param {Integer} k
# @return {Integer}
def time_required_to_buy(tickets, k)
  seconds = 0

  while true
    for i in 0...tickets.size
      if tickets[i] > 0
        seconds += 1
        tickets[i] -= 1

        return seconds if i == k && tickets[i] == 0
      end
    end
  end

  seconds
end
