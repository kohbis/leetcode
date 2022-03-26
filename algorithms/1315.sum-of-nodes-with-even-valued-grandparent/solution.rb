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
def sum_even_grandparent(root)
  @sum = 0

  sum_if_even_grandchild(root, false, false)

  @sum
end

def sum_if_even_grandchild(node, even_grapndparent, even_parent)
  return if node.nil?

  @sum += node.val if even_grapndparent

  sum_if_even_grandchild(node.left, even_parent, node.val % 2 == 0)
  sum_if_even_grandchild(node.right, even_parent, node.val % 2 == 0)
end
