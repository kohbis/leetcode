require 'set'

# @param {String[]} tokens
# @return {Integer}
def eval_rpn(tokens)
  operators = Set.new(['+', '-', '*', '/'])

  stack = []
  tokens.each do |token|
    if operators.include?(token)
      right = stack.pop.to_i
      left = stack.pop.to_i
      result = calc(left, right, token)
      stack << result
    else
      stack << token.to_i
    end
  end

  stack[0]
end

def calc(left, right, operator)
  case operator
  when '+'
    left + right
  when '-'
    left - right
  when '*'
    left * right
  when '/'
    (left.to_f / right).to_i
  else
    0
  end
end
