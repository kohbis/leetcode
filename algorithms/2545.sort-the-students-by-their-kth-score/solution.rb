# @param {Integer[][]} score
# @param {Integer} k
# @return {Integer[][]}
def sort_the_students(score, k)
  score.sort { |a, b| b[k] <=> a[k] }
end
