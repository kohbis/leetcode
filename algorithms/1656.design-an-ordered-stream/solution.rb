class OrderedStream
=begin
    :type n: Integer
=end
  def initialize(n)
    @stream = Array.new(n)
    @ptr = 0
  end

=begin
    :type id: Integer
    :type value: String
    :rtype: String[]
=end
  def insert(id, value)
    chunk_list = []
    @stream[id - 1] = value

    while @stream[@ptr]
      chunk_list.push(@stream[@ptr])
      @ptr += 1
    end

    chunk_list
  end
end

# Your OrderedStream object will be instantiated and called as such:
# obj = OrderedStream.new(n)
# param_1 = obj.insert(id, value)
#
