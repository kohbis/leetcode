# Below is the interface for Iterator, which is already defined for you.
#
# class Iterator
# 	def initialize(v)
#
#   end
#
#   def hasNext()
#		Returns true if the iteration has more elements.
#   end
#
#   def next()
#   	Returns the next element in the iteration.
#   end
# end

class PeekingIterator
  # @param {Iterator} iter
  def initialize(iter)
    @iter = iter
    @next_val = nil
  end

  # Returns true if the iteration has more elements.
  # @return {boolean}
  def hasNext()
    !@next_val.nil? || @iter.hasNext
  end

  # Returns the next element in the iteration.
  # @return {integer}
  def next()
    el = peek()
    @next_val = nil

    el
  end

  # Returns the next element in the iteration without advancing the iterator.
  # @return {integer}
  def peek()
    @next_val ||= @iter.next
  end
end
