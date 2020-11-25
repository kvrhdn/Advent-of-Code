async function load_day05() {
  let { part1, part2 } = await import(/* webpackChunkName: "day05" */ '../../day05/pkg/day05');
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day05();
