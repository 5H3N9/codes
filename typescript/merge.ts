function merge(intervals: number[][]): number[][] {
  let il = intervals.length;
  let ans: number[][] = new Array();
  intervals.sort((a: number[], b: number[])=> a[0]-b[0]);
  for (const i of intervals) {
    if (ans.length > 0 && i[0]<=ans[ans.length-1][1]) {
      ans[ans.length-1][1] = Math.max(ans[ans.length-1][1], i[1]);
    } else {
      ans.push(i);
    }
  }
  return ans;  
};
