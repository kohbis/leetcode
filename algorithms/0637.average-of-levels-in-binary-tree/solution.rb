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
# @return {Float[]}
def average_of_levels(root)
  @deepest = 0
  # [count, sum]
  @sums = Hash.new

  add_val_with_level(root, 0)

  (0..@deepest).map { |i| @sums[i][1].fdiv(@sums[i][0]).floor(5) }
end

def add_val_with_level(node, level)
  return if node.nil?

  @deepest = [@deepest, level].max

  @sums[level] = [0, 0] unless @sums.has_key?(level)
  @sums[level][0] += 1
  @sums[level][1] += node.val

  add_val_with_level(node.left, level + 1) if node.left
  add_val_with_level(node.right, level + 1) if node.right
end
