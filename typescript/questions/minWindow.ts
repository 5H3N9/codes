function minWindow(s: string, t: string): string {
  const sl = s.length;
  let mp: Map<string, number> = new Map();
  for (const c of t) {
    mp.set(c, (mp.get(c) ?? 0) + 1);
  }
  let l = 0;
  let d = mp.size;
  let ms = sl;
  let ml = ms + 1;
  for (let r = 0; r < sl; ++r) {
    const rc = s[r];
    if (mp.has(rc)) {
      mp.set(rc, mp.get(rc)! - 1);
      if (mp.get(rc) === 0) {
        --d;
      }
      if (d == 0) {
        while (d == 0) {
          const lc = s[l];
          if (mp.has(lc)) {
            mp.set(lc, mp.get(lc)! + 1);
            if (mp.get(lc)! > 0) {
              ++d;
            }
          }
          ++l;
        }
        const cl = r + 2 - l;
        if (cl < ml) {
          ms = l - 1;
          ml = cl;
        }
      }
    }
  }
  return ms == sl ? "" : s.substring(ms, ms + ml);
}
