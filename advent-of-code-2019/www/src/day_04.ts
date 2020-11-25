async function load_day04() {
  let { part1, part2 } = await import(/* webpackChunkName: "day04" */ '../../day04/pkg/day04');
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day04();
