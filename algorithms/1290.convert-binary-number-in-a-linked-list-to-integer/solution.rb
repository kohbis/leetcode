# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @return {Integer}
def get_decimal_value(head)
  decimal_value = 0
  curt = head
  while curt
    decimal_value = (decimal_value << 1) | curt.val
    curt = curt.next
  end
  decimal_value
end
