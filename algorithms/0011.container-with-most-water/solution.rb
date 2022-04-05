# @param {Integer[]} height
# @return {Integer}
def max_area(height)
  max = 0
  left, right = 0, height.size - 1

  while left < right
    amount = [height[left], height[right]].min * (right - left)
    if amount > max
      max = amount
    end

    if height[left] > height[right]
      right -= 1
    else
      left += 1
    end
  end

  max
end
