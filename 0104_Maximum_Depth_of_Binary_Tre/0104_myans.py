# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def traceNode(self, parent: Optional[TreeNode], level: int) -> int:
        if parent == None:
            return level
        
        leftlevel = self.traceNode(parent.left, level+1)
        rightlevel = self.traceNode(parent.right, level+1)

        return max(leftlevel, rightlevel)

    def maxDepth(self, root: Optional[TreeNode]) -> int:
        level = 0
        return self.traceNode(root, level)
        