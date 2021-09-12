# @param {String[]} logs
# @return {Integer}
def min_operations(logs)
  depth = 0
  logs.each do |log|
    if "../" == log
      depth -= 1 if depth > 0
    elsif "./" != log
      depth += 1
    end
  end

  depth
end
