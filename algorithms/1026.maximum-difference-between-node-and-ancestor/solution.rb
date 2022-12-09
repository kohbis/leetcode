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
def max_ancestor_diff(root)
  max_diff(root, root.val, root.val)
end

def max_diff(root, max, min)
  return max - min if root.nil?

  max = [max, root.val].max
  min = [min, root.val].min

  left = max_diff(root.left, max, min)
  right = max_diff(root.right, max, min)

  [left, right].max
end
