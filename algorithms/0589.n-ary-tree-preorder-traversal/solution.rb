# Definition for a Node.
# class Node
#     attr_accessor :val, :children
#     def initialize(val)
#         @val = val
#         @children = []
#     end
# end

# @param {Node} root
# @return {List[int]}
def preorder(root)
  res = []

  return res if root.nil?

  res.push root.val

  root.children.each do |child|
    res.concat preorder(child)
  end

  res
end
