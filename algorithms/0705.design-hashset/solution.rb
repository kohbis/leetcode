class MyHashSet

=begin
  Initialize your data structure here.
=end
  def initialize()
    @container = Array.new()
  end

=begin
  :type key: Integer
  :rtype: Void
=end
  def add(key)
    @container[key] = true
  end

=begin
  :type key: Integer
  :rtype: Void
=end
  def remove(key)
    @container[key] = nil
  end

=begin
  Returns true if this set contains the specified element
  :type key: Integer
  :rtype: Boolean
=end
  def contains(key)
    !!@container[key]
  end
end

# Your MyHashSet object will be instantiated and called as such:
# obj = MyHashSet.new()
# obj.add(key)
# obj.remove(key)
# param_3 = obj.contains(key)
