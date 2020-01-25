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
def add_two_numbers(l1, l2)
  val = l1.val + l2.val
  res = ListNode.new(val % 10)
  curt_res = res

  over = val >= 10 ? 1 : 0
  l1 = l1.next
  l2 = l2.next

  while over > 0 || !l1.nil? || !l2.nil?
    val = over
    val += l1.nil? ? 0 : l1.val
    val += l2.nil? ? 0 : l2.val

    curt_res.next = ListNode.new(val % 10)
    
    over = val >= 10 ? 1 : 0
    l1 = l1.nil? ? l1 : l1.next
    l2 = l2.nil? ? l2 : l2.next
    curt_res = curt_res.next
  end

  res
end
