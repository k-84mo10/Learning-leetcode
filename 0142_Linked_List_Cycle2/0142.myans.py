# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head == None:
            return 

        express = head
        local = head

        while True:
            if express.next == None:
                return

            if express.next.next == None:
                return
    
            express = express.next.next
            local = local.next
            
            if express == local:
                break

        express = head
        while True:
            if express == local:
                return local

            express = express.next
            local = local.next

