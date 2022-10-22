class MyQueue
  def initialize()
    @queue = []
  end

=begin
    :type x: Integer
    :rtype: Void
=end
  def push(x)
    @queue << x
  end

=begin
    :rtype: Integer
=end
  def pop()
    @queue.shift
  end

=begin
    :rtype: Integer
=end
  def peek()
    @queue.first
  end

=begin
    :rtype: Boolean
=end
  def empty()
    @queue.empty?
  end
end

# Your MyQueue object will be instantiated and called as such:
# obj = MyQueue.new()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.peek()
# param_4 = obj.empty()
