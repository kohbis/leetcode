/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        int val = l1.val + l2.val;
        ListNode res = new ListNode(val % 10);
        ListNode current_res = res;

        int over = (val >= 10) ? 1 : 0;
        l1 = l1.next;
        l2 = l2.next;

        while (over > 0 || l1 != null || l2 != null) {
            val = over;
            val += (null == l1) ? 0 : l1.val;
            val += (null == l2) ? 0 : l2.val;

            current_res.next = new ListNode(val % 10);

            over = (val >= 10) ? 1 : 0;
            l1 = (null == l1) ? l1 : l1.next;
            l2 = (null == l2) ? l2 : l2.next;
            current_res = current_res.next;
        }

        return res;
    }
}
