class UndergroundSystem
  def initialize()
    # { start_station => end_station => [time] }
    @checks = Hash.new { |h, k| h[k] = Hash.new { |h, k| h[k] = [] } }

    # { [start_station, check_in_time] }
    @customers = Hash.new { |h, k| h[k] = [] }
  end

=begin
  :type id: Integer
  :type station_name: String
  :type t: Integer
  :rtype: Void
=end
  def check_in(id, station_name, t)
    @customers[id] = [station_name, t]
  end

=begin
  :type id: Integer
  :type station_name: String
  :type t: Integer
  :rtype: Void
=end
  def check_out(id, station_name, t)
    start_station, check_in_time = @customers[id]
    @checks[start_station][station_name] << t - check_in_time
  end

=begin
  :type start_station: String
  :type end_station: String
  :rtype: Float
=end
  def get_average_time(start_station, end_station)
    times = @checks[start_station][end_station]
    times.sum.to_f / times.size
  end
end

# Your UndergroundSystem object will be instantiated and called as such:
# obj = UndergroundSystem.new()
# obj.check_in(id, station_name, t)
# obj.check_out(id, station_name, t)
# param_3 = obj.get_average_time(start_station, end_station)
