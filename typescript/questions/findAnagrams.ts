function findAnagrams(s: string, p: string): number[] {
  let pl = p.length;
  let pmp: Map<string, number> = new Map();
  let ans: number[] = [];
  let l = 0;
  let r = 0;
  for (const c of p) {
    pmp.set(c, (pmp.get(c) ?? 0) + 1);
  }
  for (; r < s.length; ++r) {
    const rc = s[r];
    pmp.set(rc, (pmp.get(rc) ?? 0) - 1);
    while (pmp.get(rc)! < 0) {
      pmp.set(s[l], (pmp.get(s[l]) ?? 0) + 1);
      ++l;
    }
    if (r - l === pl - 1) {
      ans.push(l);
    }
  }
  return ans;
}
