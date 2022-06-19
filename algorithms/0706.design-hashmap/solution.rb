class MyHashMap
  def initialize()
    @data = {}
  end

=begin
    :type key: Integer
    :type value: Integer
    :rtype: Void
=end
  def put(key, value)
    @data[key] = value
  end

=begin
    :type key: Integer
    :rtype: Integer
=end
  def get(key)
    @data[key] || -1
  end

=begin
    :type key: Integer
    :rtype: Void
=end
  def remove(key)
    @data[key] = -1
  end
end

# Your MyHashMap object will be instantiated and called as such:
# obj = MyHashMap.new()
# obj.put(key, value)
# param_2 = obj.get(key)
# obj.remove(key)
