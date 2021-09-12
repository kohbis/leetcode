# @param {String[][]} paths
# @return {String}
def dest_city(paths)
  a, b = [], []
  paths.each do |path|
    # departures
    a << path[0]
    # destinations
    b << path[1]
  end
  b.find { |d| !a.include?(d) }
end
