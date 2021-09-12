# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} l1
# @param {ListNode} l2
# @return {ListNode}
def merge_two_lists(l1, l2)
  res = []
  while !l1.nil? || !l2.nil?
    res << l1.val unless l1.nil?
    res << l2.val unless l2.nil?

    l1 = l1.nil? ? l1 : l1.next
    l2 = l2.nil? ? l2 : l2.next
  end
  res.sort
end
