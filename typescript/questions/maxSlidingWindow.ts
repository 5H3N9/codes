function maxSlidingWindow(nums: number[], k: number): number[] {
  let nl = nums.length;
  let ans: number[] = new Array(nl - k + 1);
  let dq: number[] = [];
  for (let p = 0; p < nl; ++p) {
    while (dq.length && nums[p] > nums[dq[dq.length - 1]]) {
      dq.pop();
    }
    dq.push(p);
    let l = p - k + 1;
    if (dq[0] < l) {
      dq.shift();
    }
    if (l >= 0) {
      ans[l] = nums[dq[0]];
    }
  }
  return ans;
}
