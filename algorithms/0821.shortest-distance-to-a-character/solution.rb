# @param {String} s
# @param {Character} c
# @return {Integer[]}
def shortest_to_char(s, c)
  res = []
  index_queue = []

  last_c_idx = nil

  s.chars.each_with_index do |char, idx|
    if char == c
      while index_queue.any?
        last_c_idx = idx unless last_c_idx
        first = index_queue.shift
        res.push [idx - first, (last_c_idx - first).abs].min
      end
      res.push 0
      last_c_idx = idx
    else
      index_queue.push idx
    end
  end

  while index_queue.any?
    first = index_queue.shift
    res.push (last_c_idx - first).abs
  end

  res
end
