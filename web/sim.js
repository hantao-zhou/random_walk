export function randomWalk(steps) {
  const x = [0];
  const y = [0];
  let cx = 0;
  let cy = 0;
  for (let i = 0; i < steps; i++) {
    const dir = Math.floor(Math.random() * 4);
    switch (dir) {
      case 0:
        cx += 1;
        break;
      case 1:
        cx -= 1;
        break;
      case 2:
        cy += 1;
        break;
      default:
        cy -= 1;
        break;
    }
    x.push(cx);
    y.push(cy);
  }
  return { x, y };
}

export function edgeReinforcedWalk(steps, strength, delay, memory) {
  const x = [0];
  const y = [0];
  let cx = 0;
  let cy = 0;
  const counts = new Map();
  const history = [];

  const edgeKey = (a, b) => {
    const first = a[0] < b[0] || (a[0] === b[0] && a[1] <= b[1]) ? a : b;
    const second = first === a ? b : a;
    return `${first[0]},${first[1]}-${second[0]},${second[1]}`;
  };

  for (let step = 0; step < steps; step++) {
    const neighbors = [
      [cx + 1, cy],
      [cx - 1, cy],
      [cx, cy + 1],
      [cx, cy - 1]
    ];
    const weights = [];
    let total = 0;
    for (let i = 0; i < 4; i++) {
      const key = edgeKey([cx, cy], neighbors[i]);
      const count = counts.get(key) || 0;
      const w = step >= delay ? 1 + strength * count : 1;
      weights.push(w);
      total += w;
    }
    let r = Math.random() * total;
    let choice = 0;
    for (let i = 0; i < 4; i++) {
      if (r < weights[i]) {
        choice = i;
        break;
      }
      r -= weights[i];
    }
    const next = neighbors[choice];
    const key = edgeKey([cx, cy], next);
    counts.set(key, (counts.get(key) || 0) + 1);
    history.push(key);
    if (memory > 0 && history.length > memory) {
      const old = history.shift();
      const c = counts.get(old) || 0;
      if (c > 0) counts.set(old, c - 1);
    }
    cx = next[0];
    cy = next[1];
    x.push(cx);
    y.push(cy);
  }
  return { x, y };
}
