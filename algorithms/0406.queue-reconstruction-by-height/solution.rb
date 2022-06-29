# @param {Integer[][]} people
# @return {Integer[][]}
def reconstruct_queue(people)
  people.sort_by { |a, b| [-a, b] }.each_with_object([]) { |obj, arr| arr.insert(obj[1], obj) }
end
