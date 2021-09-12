# @param {Integer} num
# @return {Integer}
def number_of_steps(num)
  step = 0
  while num > 0
    num = num.even? ? num / 2 : num - 1
    step += 1
  end
  step
end
