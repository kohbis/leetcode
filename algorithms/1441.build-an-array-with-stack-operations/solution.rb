# @param {Integer[]} target
# @param {Integer} n
# @return {String[]}
def build_array(target, n)
  arr = []

  for i in 1..target[-1]
    arr << "Push"
    unless target.include?(i)
      arr << "Pop"
    end
  end

  arr
end
