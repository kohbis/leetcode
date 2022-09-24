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
# @param {Integer} target_sum
# @return {Integer[][]}
def path_sum(root, target_sum)
  @res = []
  helper(root, target_sum, [])
  @res
end

def helper(node, sum, current)
  return if node.nil?

  sum -= node.val

  @res << current + [node.val] if sum == 0 && node.left.nil? && node.right.nil?

  helper(node.left, sum, current + [node.val]) if node.left
  helper(node.right, sum, current + [node.val]) if node.right
end
