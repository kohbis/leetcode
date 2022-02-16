# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @return {ListNode}
def swap_pairs(head)
  if head&.next
    tmp = head.next.next
    head, head.next.next = head.next, head
    head.next.next = swap_pairs(tmp)
  end

  head
end
