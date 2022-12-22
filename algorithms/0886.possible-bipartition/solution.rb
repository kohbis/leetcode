require 'set'
# @param {Integer} n
# @param {Integer[][]} dislikes
# @return {Boolean}
def possible_bipartition(n, dislikes)
  group1 = Set.new
  group2 = Set.new

  dislikes.sort! { |a, b| a[0] <=> b[0] }
  dislikes.each do |a, b|
    a1 = group1.include?(a)
    b1 = group1.include?(b)
    a2 = group2.include?(a)
    b2 = group2.include?(b)

    return false if (a1 && b1) || (a2 && b2)

    if !a1 && !b1 && !a2 && !b2
      group1 = [a]
      group2 = [b]
      next
    end

    group2 << b if a1 && !b2
    group2 << a if b1 && !a2
    group1 << b if a2 && !b1
    group1 << a if b2 && !a1
  end

  true
end
