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
# @return {Boolean}
def evaluate_tree(root)
  # 0: False, 1: True, 2: OR, 3: AND
  return root.val == 1 if root.val < 2
  l = evaluate_tree(root.left)
  r = evaluate_tree(root.right)
  root.val == 2 ? (l | r) : (l & r)
end
