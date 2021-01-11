# @param {Integer[]} candidates
# @param {Integer} target
# @return {Integer[][]}
def combination_sum(candidates, target)
  generate_combination(candidates, target)
end

def generate_combination(candidates, target, current = [], index = 0, results = [])
  sum = current.sum

  return if sum > target
  return results.push(current.clone) if sum == target

  (index...candidates.size).each do |i|
    next if sum + candidates[i] > target

    current.push(candidates[i])
    generate_combination(candidates, target, current, i, results)
    current.pop
  end

  results
end

