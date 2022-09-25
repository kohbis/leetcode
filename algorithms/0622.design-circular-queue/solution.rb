class MyCircularQueue

=begin
    :type k: Integer
=end
  def initialize(k)
    @queue = Array.new(k)
    @front = 0
    @rear = 0
  end

=begin
    :type value: Integer
    :rtype: Boolean
=end
  def en_queue(value)
    return false if self.is_full

    @queue[@rear] = value
    if @rear == @queue.size - 1
      @rear = 0
    else
      @rear += 1
    end
    true
  end

=begin
    :rtype: Boolean
=end
  def de_queue()
    return false if self.is_empty

    @queue[@front] = nil
    if @front == @queue.size - 1
      @front = 0
    else
      @front += 1
    end
    true
  end

=begin
    :rtype: Integer
=end
  def front()
    self.is_empty ? -1 : @queue[@front]
  end

=begin
    :rtype: Integer
=end
  def rear()
    self.is_empty ? -1 : @queue[@rear - 1]
  end

=begin
    :rtype: Boolean
=end
  def is_empty()
    @queue.count { _1 } == 0
  end

=begin
    :rtype: Boolean
=end
  def is_full()
    @queue.count { _1 } == @queue.size
  end
end

# Your MyCircularQueue object will be instantiated and called as such:
# obj = MyCircularQueue.new(k)
# param_1 = obj.en_queue(value)
# param_2 = obj.de_queue()
# param_3 = obj.front()
# param_4 = obj.rear()
# param_5 = obj.is_empty()
# param_6 = obj.is_full()
