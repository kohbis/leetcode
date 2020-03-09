# @param {Integer} num
# @return {Integer}
def number_of_steps (num)
  times = 0
  loop do
    num = num.even? ? num / 2 : num -1
    times += 1
    break if num == 0
  end
  times
end
