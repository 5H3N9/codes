#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int maxs = nums[0];
        int curs = nums[0];
        for (int i=1; i<nums.size(); ++i) {
            curs = std::max(nums[i], nums[i]+curs);
            maxs = std::max(maxs, curs);
        }
        return maxs;
    }
};
