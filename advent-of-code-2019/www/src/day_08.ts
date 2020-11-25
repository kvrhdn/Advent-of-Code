async function load_day08() {
  let { part1, part2 } = await import(/* webpackChunkName: "day08" */ '../../day08/pkg/day08');
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day08();
