/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution
{
public:
    vector<vector<int>> spiralMatrix(int m, int n, ListNode *head)
    {
        vector<vector<int>> spiral_matrix(m, vector<int>(n, -1));
        int vec = 0;
        int i = 0;
        int j = 0;

        while (head != nullptr)
        {
            spiral_matrix[i][j] = head->val;
            if (vec == 0)
            {
                if (j == n - 1 || spiral_matrix[i][j + 1] != -1)
                {
                    vec = 1;
                    i += 1;
                }
                else
                {
                    j += 1;
                }
            }
            else if (vec == 1)
            {
                if (i == m - 1 || spiral_matrix[i + 1][j] != -1)
                {
                    vec = 2;
                    j -= 1;
                }
                else
                {
                    i += 1;
                }
            }
            else if (vec == 2)
            {
                if (j == 0 || spiral_matrix[i][j - 1] != -1)
                {
                    vec = 3;
                    i -= 1;
                }
                else
                {
                    j -= 1;
                }
            }
            else
            {
                if (i == 0 || spiral_matrix[i - 1][j] != -1)
                {
                    vec = 0;
                    j += 1;
                }
                else
                {
                    i -= 1;
                }
            }

            head = head->next;
        }

        return spiral_matrix;
    }
};