# @param {Integer} length
# @param {Integer} width
# @param {Integer} height
# @param {Integer} mass
# @return {String}
def categorize_box(length, width, height, mass)
  vol = length * width * height
  is_bulky = [length, width, height].any? { |v| v >= 10 ** 4 } || vol >= 10 ** 9
  is_heavy = mass >= 100

  case [is_bulky, is_heavy]
  when [true, true]
    'Both'
  when [false, false]
    'Neither'
  when [true, false]
    'Bulky'
  when [false, true]
    'Heavy'
  end
end
