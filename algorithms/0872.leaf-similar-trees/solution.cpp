#include <vector>

using namespace std;

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    bool leafSimilar(TreeNode* root1, TreeNode* root2) {
        vector<int> seqLeft;
        vector<int> seqRight;

        leafSequence(root1, seqLeft);
        leafSequence(root2, seqRight);

        return seqLeft == seqRight;
    }

private:
    void leafSequence(TreeNode* root, vector<int>& current) {
        if (root) {
            if (root->left == nullptr && root->right == nullptr) {
                current.push_back(root->val);
            }
            leafSequence(root->left, current);
            leafSequence(root->right, current);
        }
    }
};
