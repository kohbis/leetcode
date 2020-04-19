# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val)
#         @val = val
#         @left, @right = nil, nil
#     end
# end

# @param {TreeNode} root
# @return {Integer}
def diameter_of_binary_tree(root)
  @longest = 0
  depth(root) unless root.nil?
  @longest
end

def depth(node)
  return 0 if node.nil?

  left = depth(node.left)
  right = depth(node.right)
  @longest = [@longest, left + right].max

  [left, right].max + 1
end

