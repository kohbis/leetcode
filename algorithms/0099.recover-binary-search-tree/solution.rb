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
# @return {Void} Do not return anything, modify root in-place instead.
def recover_tree(root)
  @vals = []
  nodeVals(root)
  @vals = @vals.sort.reverse
  recover(root)
end

def nodeVals(node)
  return if node.nil?
  @vals << node.val
  nodeVals(node.left)
  nodeVals(node.right)
end

def recover(node)
  return if node.nil?

  recover(node.left)
  node.val = @vals.pop
  recover(node.right)
end
