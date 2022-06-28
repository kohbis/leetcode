# @param {String} time
# @return {String}
def maximum_time(time)
  res = (time[0] != '?' ? time[0] : (time[1] == '?' || time[1] < '4') ? '2' : '1')
  res << (time[1] != '?' ? time[1] : res[0] == '2' ? '3' : '9')
  res << ':'
  res << (time[3] != '?' ? time[3] : '5')
  res << (time[4] != '?' ? time[4] : '9')

  res
end
