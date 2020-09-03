# @param {String} s
# @return {String}
def sort_string(s)
  res = []
  seeds = s.chars.sort
  rev = false

  while seeds.any?
    additions = rev ? seeds.uniq.reverse : seeds.uniq
    res += additions

    additions.each do |c|
      seeds.delete_at(seeds.find_index(c))
    end

    rev = !rev
  end

  res.join
end
