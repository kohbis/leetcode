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
def increasing_bst(root)
  nodes = build_node_array(root)

  for i in 0...nodes.size
    nodes[i].left = nil
    nodes[i].right = nodes[i + 1]
  end

  nodes[0]
end

def build_node_array(root)
  return [] unless root
  build_node_array(root.left) + [root] + build_node_array(root.right)
end
