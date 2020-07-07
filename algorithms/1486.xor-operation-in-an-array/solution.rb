# @param {Integer} n
# @param {Integer} start
# @return {Integer}
def xor_operation(n, start)
  (1...n).inject(start) {|res, i| res ^ (start + 2*i) }
end
