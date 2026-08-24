function subarraySum(nums: number[], k: number): number {
  let nl = nums.length;
  let mp: Map<number, number> = new Map();
  let ps = 0;
  let ans = 0;
  mp.set(ps, (mp.get(ps) ?? 0) + 1);
  for (let p = 0; p < nl; ++p) {
    ps += nums[p];
    ans += mp.get(ps - k) ?? 0;
    mp.set(ps, (mp.get(ps) ?? 0) + 1);
  }
  return ans;
}
