# @param {Integer[][]} points
# @return {Integer}
def max_width_of_vertical_area(points)
  xs = points.map { |p| p[0] }.sort.uniq

  res = 0
  for i in 0...xs.size - 1
    res = [res, xs[i + 1] - xs[i]].max
  end

  res
end
