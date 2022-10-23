# @param {String[]} garbage
# @param {Integer[]} travel
# @return {Integer}
def garbage_collection(garbage, travel)
  t_metal = 0
  t_paper = 0
  t_glass = 0

  total_time = 0
  travel.each_with_index do |t, i|
    total_time += t
    t_metal = total_time if garbage[i + 1].include?('M')
    t_paper = total_time if garbage[i + 1].include?('P')
    t_glass = total_time if garbage[i + 1].include?('G')
  end

  garbage.map(&:length).sum + t_metal + t_paper + t_glass
end
