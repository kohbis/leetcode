# @param {Integer[][]} points
# @return {Integer}
def min_time_to_visit_all_points(points)
  seconds = 0
  points.each_with_index do |p, i|
    break if (points.size-1) == i
    seconds += [(p[0] - points[i+1][0]).abs, (p[1] - points[i+1][1]).abs].max
  end
  seconds
end

