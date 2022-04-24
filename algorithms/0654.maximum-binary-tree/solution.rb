# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {Integer[]} nums
# @return {TreeNode}
def construct_maximum_binary_tree(nums)
  case nums.size
  when 0
    nil
  when 1
    TreeNode.new(nums[0])
  else
    max, max_index = nums.each_with_index.max
    left = nums[0...max_index]
    right = nums[max_index + 1..]
    TreeNode.new(max, construct_maximum_binary_tree(left), construct_maximum_binary_tree(right))
  end
end
