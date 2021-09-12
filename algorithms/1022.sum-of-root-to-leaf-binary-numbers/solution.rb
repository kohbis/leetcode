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
def sum_root_to_leaf(root)
  @sum = 0
  dfs(root, "") if root
  @sum
end

def dfs(root, current)
  return unless root

  current += root.val.to_s

  if root.left.nil? && root.right.nil?
    @sum += current.to_i(2)
  end

  dfs(root.left, current)
  dfs(root.right, current)

  current.chop
end
