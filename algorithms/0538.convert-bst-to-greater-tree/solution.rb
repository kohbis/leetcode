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
# @return {TreeNode}
def convert_bst(root)
  @sum = 0
  build_greater_tree(root)
end

def build_greater_tree(node)
  return nil if node.nil?

  build_greater_tree(node.right)

  @sum += node.val
  node.val = @sum

  build_greater_tree(node.left)

  node
end
