# @param {Integer[]} seats
# @param {Integer[]} students
# @return {Integer}
def min_moves_to_seat(seats, students)
  seats.sort!
  students.sort!

  res = 0
  (0...seats.size).each do |i|
    res += (seats[i] - students[i]).abs
  end

  res
end
