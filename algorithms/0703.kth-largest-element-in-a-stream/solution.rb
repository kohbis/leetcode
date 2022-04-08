class KthLargest

=begin
    :type k: Integer
    :type nums: Integer[]
=end
  def initialize(k, nums)
    @size = k
    @elements = nums.sort { |a, b| b <=> a }[0...k]
  end

=begin
    :type val: Integer
    :rtype: Integer
=end
  def add(val)
    index = @elements.bsearch_index { |x| x < val }
    if index
      @elements.insert(index, val)
    else
      @elements.push(val)
    end

    @elements.pop if @elements.size > @size

    @elements.last
  end
end

# Your KthLargest object will be instantiated and called as such:
# obj = KthLargest.new(k, nums)
# param_1 = obj.add(val)
