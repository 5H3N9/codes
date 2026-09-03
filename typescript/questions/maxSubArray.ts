function maxSubArray(nums: number[]): number {
  let max = nums[0];
  let cur = nums[0];
  for (let i=1; i<nums.length; ++i) {
    cur = Math.max(cur+nums[i], nums[i]);
    max = Math.max(cur, max);
  }
  return max;
};
