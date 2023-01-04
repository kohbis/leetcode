# @param {Integer[]} tasks
# @return {Integer}
def minimum_rounds(tasks)
  count = tasks.tally
  rounds = 0
  count.each do |level, count|
    return -1 if count < 2

    rounds += case count % 3
      when 0
        count / 3
      when 1
        (count - 4) / 3 + 2
      when 2
        (count - 2) / 3 + 1
      end
  end
  rounds
end
