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
def merge_nodes(head)
  ans = []

  sum = 0
  while head != nil
    if head.val == 0
      if sum > 0
        ans << sum
        sum = 0
      end
    else
      sum += head.val
    end

    head = head.next
  end

  ans
end
