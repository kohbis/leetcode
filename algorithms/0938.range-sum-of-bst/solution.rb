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
# @param {Integer} l
# @param {Integer} r
# @return {Integer}
def range_sum_bst(root, l, r)
  res = 0
  return res if root.nil?
  res = root.val if  l <= root.val && root.val <= r
  res + range_sum_bst(root.left, l, r) + range_sum_bst(root.right, l, r)
end
