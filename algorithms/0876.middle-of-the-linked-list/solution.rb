# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} head
# @return {ListNode}
def middle_node(head)
  tmp, i = [], 0
  while (head)
    tmp[i] = head.val
    i += 1
    head = head.next
  end
  tmp[i / 2..-1]
end
