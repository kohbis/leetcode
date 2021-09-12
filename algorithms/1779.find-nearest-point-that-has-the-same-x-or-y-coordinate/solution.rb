# @param {Integer} x
# @param {Integer} y
# @param {Integer[][]} points
# @return {Integer}
def nearest_valid_point(x, y, points)
  dists = []

  points.each_with_index do |point, idx|
    px, py = point
    if x == px || y == py
      dists << [(x - px).abs + (y - py).abs, idx]
    end
  end

  dists.empty? ? -1 : dists.sort[0][1]
end
