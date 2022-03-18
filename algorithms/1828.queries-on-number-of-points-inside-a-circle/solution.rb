# @param {Integer[][]} points
# @param {Integer[][]} queries
# @return {Integer[]}
def count_points(points, queries)
  queries.map do |query|
    points.count { |point| distance(query, point) <= query[2] }
  end
end

def distance(point1, point2)
  xd = (point2[0] - point1[0]).pow(2)
  yd = (point2[1] - point1[1]).pow(2)
  Math.sqrt(xd + yd)
end
