# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
class BSTIterator

=begin
    :type root: TreeNode
=end
  def initialize(root)
    @root = root
  end

=begin
    :rtype: Integer
=end
  def next()
    return nil if @root.nil?

    pt, pre = @root, nil
    while pt.left
      pre = pt
      pt = pt.left
    end

    if pt.right
      if pre.nil?
        @root = pt.right
      else
        pre.left = pt.right
      end
    else
      if pre.nil?
        @root = nil
      else
        pre.left = nil
      end
    end

    pt.val
  end

=begin
    :rtype: Boolean
=end
  def has_next()
    !@root.nil?
  end
end

# Your BSTIterator object will be instantiated and called as such:
# obj = BSTIterator.new(root)
# param_1 = obj.next()
# param_2 = obj.has_next()
