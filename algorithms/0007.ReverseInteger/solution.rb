# @param {Integer} x
# @return {Integer}
def reverse(x)
  res = x.abs.to_s.reverse.to_i
  res = -res if x.negative?
  (res >= -2_147_483_648 && res <= 21_47_483_647) ? res : 0
end
