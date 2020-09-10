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
def postorder(root)
  res = []

  return res if root.nil?

  root.children.each do |child|
    res.concat postorder(child)
  end

  res << root.val
end
