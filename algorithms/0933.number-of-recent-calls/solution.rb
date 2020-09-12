class RecentCounter
  def initialize()
    @pings = []
  end

=begin
  :type t: Integer
  :rtype: Integer
=end
  def ping(t)
    @pings << t
    @pings.shift while @pings[0] < t - 3000
    @pings.size
  end
end

# Your RecentCounter object will be instantiated and called as such:
# obj = RecentCounter.new()
# param_1 = obj.ping(t)
