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
# @param {Integer} x
# @param {Integer} y
# @return {Boolean}
def is_cousins(root, x, y)
  x_parent, x_depth = parent_and_depth(root, x, nil, 0)
  y_parent, y_depth = parent_and_depth(root, y, nil, 0)

  (x_parent != y_parent) && (x_depth == y_depth)
end

# Find parent and depth of node
# @param {TreeNode} root
# @param {Integer} target
# @param {Integer} parent
# @param {Integer} depth
# @return {Integer, Integer} parent, depth
def parent_and_depth(root, target, parent, depth)
  return if root.nil?
  return [parent, depth] if root.val == target

  depth += 1

  left_parent, left_depth = parent_and_depth(root.left, target, root, depth)
  return [left_parent, left_depth] if left_depth

  right_parent, right_depth = parent_and_depth(root.right, target, root, depth)
  return [right_parent, right_depth] if right_depth
end

