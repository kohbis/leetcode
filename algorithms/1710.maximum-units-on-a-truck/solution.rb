# @param {Integer[][]} box_types
# @param {Integer} truck_size
# @return {Integer}
def maximum_units(box_types, truck_size)
  res = 0

  box_types.sort { |a, b| b[1] <=> a[1] }.each do |num, unit|
    if num > truck_size
      res += truck_size * unit
      break
    else
      res += num * unit
      truck_size -= num
    end
  end

  res
end

