# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @param {Integer} k
# @return {ListNode}
def swap_nodes(head, k)
  curr = head

  (k - 1).times { curr = curr.next }

  a = curr
  b = head
  curr = curr.next

  while curr
    curr, b = curr.next, b.next
  end

  a.val, b.val = b.val, a.val
  head
end
