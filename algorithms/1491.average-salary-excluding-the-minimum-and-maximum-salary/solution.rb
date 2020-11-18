# @param {Integer[]} salary
# @return {Float}
def average(salary)
  ### Use sort
  # salary.sort[1..-2].sum / (salary.size - 2).to_f

  min, max = salary[0], 0
  sum = 0
  salary.each do |s|
    min = s if s < min
    max = s if s > max
    sum += s
  end
  (sum - (min + max)) / (salary.size - 2).to_f
end
