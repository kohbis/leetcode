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
# @param {Integer} k
# @return {Integer}
def kth_smallest(root, k)
  @vals = []
  helper(root)
  @vals.sort[k - 1]
end

def helper(node)
  return if node.nil?

  @vals << node.val
  helper(node.left)
  helper(node.right)
end
