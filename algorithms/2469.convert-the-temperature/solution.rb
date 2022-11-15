# @param {Float} celsius
# @return {Float[]}
def convert_temperature(celsius)
  [celsius.round(5) + 273.15, celsius.round(5) * 1.80 + 32.00]
end
