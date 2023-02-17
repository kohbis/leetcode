# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {TreeNode} root
# @return {Integer}
def min_diff_in_bst(root)
  @min = nil
  @pre = nil
  helper(root)
  @min
end

def helper(node)
  return if node.nil?
  helper(node.left)
  if @pre
    diff = node.val - @pre
    @min = diff if @min.nil? || diff < @min
  end
  @pre = node.val
  helper(node.right)
end
