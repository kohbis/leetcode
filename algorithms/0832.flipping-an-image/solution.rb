# @param {Integer[][]} a
# @return {Integer[][]}
def flip_and_invert_image(a)
  a.map { |line| line.map { |bit| 1 ^ bit }.reverse }
end
