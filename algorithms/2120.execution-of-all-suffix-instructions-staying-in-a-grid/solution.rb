# @param {Integer} n
# @param {Integer[]} start_pos
# @param {String} s
# @return {Integer[]}
def execute_instructions(n, start_pos, s)
  res = []

  for i in 0...s.length
    curr_pos = start_pos.dup
    count = 0

    for j in i...s.length
      case s[j]
      when 'L'
        curr_pos[1] -= 1
      when 'R'
        curr_pos[1] += 1
      when 'U'
        curr_pos[0] -= 1
      when 'D'
        curr_pos[0] += 1
      end

      break if curr_pos[0] < 0 || n <= curr_pos[0] || curr_pos[1] < 0 || n <= curr_pos[1]

      count += 1
    end

    res << count
  end

  res
end
