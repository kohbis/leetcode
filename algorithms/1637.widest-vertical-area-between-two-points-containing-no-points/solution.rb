# @param {Integer[][]} points
# @return {Integer}
def max_width_of_vertical_area(points)
  xs = points.map { |p| p[0] }.sort.uniq

  ans = 0
  for i in 0...xs.size - 1
    ans = [ans, xs[i + 1] - xs[i]].max
  end

  ans
end
