# @param {String} date
# @return {Integer}
def day_of_year(date)
  year, month, day = date.split('-').map(&:to_i)
  days = [
    0,
    31, # Jan.
    28, # Feb.
    31, # Mar.
    30, # Apr.
    31, # May.
    30, # June
    31, # July
    31, # Aug.
    30, # Sept.
    31, # Oct.
    30, # Nov.
    31,  # Dec.
  ]
  days[2] = 29 if leap_year?(year)

  # days of previous month + current month
  days[0...month].sum + day
end

def leap_year?(year)
  (year % 4 == 0) && !(year % 100 == 0) || (year % 400 == 0)
end
