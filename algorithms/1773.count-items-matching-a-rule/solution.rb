# @param {String[][]} items
# @param {String} rule_key
# @param {String} rule_value
# @return {Integer}
def count_matches(items, rule_key, rule_value)
  index = %w(type color name).index(rule_key)
  count = 0
  items.each do |item|
    count += 1 if rule_value == item[index]
  end
  count
end
