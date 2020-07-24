# @param {Integer[]} start_time
# @param {Integer[]} end_time
# @param {Integer} query_time
# @return {Integer}
def busy_student(start_time, end_time, query_time)
  (0...start_time.size).inject(0) do |res, i|
    res += start_time[i] <= query_time && query_time <= end_time[i] ? 1 : 0
  end
end

