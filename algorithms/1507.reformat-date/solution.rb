# @param {String} date
# @return {String}
def reformat_date(date)
  ## use Date ##
  # require 'date'
  # Date.parse(date).to_s

  year, month, day = date.split(' ').reverse

  months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']
  month = (months.find_index { |m| m == month } + 1).to_s.rjust(2, '0')

  # remove st, nd, rd, th
  day = day.tr('dhnrst', '').rjust(2, '0')

  [year, month, day].join('-')
end
