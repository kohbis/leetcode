class MyStack

=begin
    Initialize your data structure here.
=end
    def initialize()
      @queue = []
      @size = 0
    end


=begin
    Push element x onto stack.
    :type x: Integer
    :rtype: Void
=end
    def push(x)
      @queue << x

      @size.times do
        @queue << @queue.shift
      end

      @size += 1

      return
    end


=begin
    Removes the element on top of the stack and returns that element.
    :rtype: Integer
=end
    def pop()
      return if @size == 0

      @size -= 1
      @queue.shift
    end


=begin
    Get the top element.
    :rtype: Integer
=end
    def top()
      @queue[0]
    end


=begin
    Returns whether the stack is empty.
    :rtype: Boolean
=end
    def empty()
      @size == 0
    end


end

# Your MyStack object will be instantiated and called as such:
# obj = MyStack.new()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.top()
# param_4 = obj.empty()
