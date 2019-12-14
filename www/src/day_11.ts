async function load_day11() {
  let { part1, part2 } = await import(/* webpackChunkName: "day11" */ '../../day11/pkg/day11');
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day11();
