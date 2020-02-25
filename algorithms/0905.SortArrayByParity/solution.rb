# @param {Integer[]} a
# @return {Integer[]}
def sort_array_by_parity(a)
  a.inject([]) do |res, num|
    if num.odd?
      res.push(num)
    else
      res.unshift(num)
    end
  end
end
