class ParkingSystem

=begin
  :type big: Integer
  :type medium: Integer
  :type small: Integer
=end
  def initialize(big, medium, small)
    @parking_spaces = [0, big, medium, small]
  end

=begin
  :type car_type: Integer
  :rtype: Boolean
=end
  def add_car(car_type)
    return false if @parking_spaces[car_type] == 0

    @parking_spaces[car_type] -= 1
    true
  end
end

# Your ParkingSystem object will be instantiated and called as such:
# obj = ParkingSystem.new(big, medium, small)
# param_1 = obj.add_car(car_type)
