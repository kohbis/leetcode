# @param {Integer} n
# @param {Integer[][]} trust
# @return {Integer}
def find_judge(n, trust)
  return 1 if trust.empty?

  people = Array.new(n + 1, 0)
  trust.each do |a, b|
    people[a] -= 1
    people[b] += 1
  end

  people.index(n - 1) || -1
end
