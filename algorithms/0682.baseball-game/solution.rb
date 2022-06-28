# @param {String[]} ops
# @return {Integer}
def cal_points(ops)
  scores = []

  ops.each do |op|
    case op
    when 'C'
      scores.pop
    when 'D'
      scores.push(scores.last * 2)
    when '+'
      scores.push(scores.last(2).sum)
    else
      scores.push(op.to_i)
    end
  end

  scores.sum
end
