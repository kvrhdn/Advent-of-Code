async function load_day16() {
  let { part1, part2 } = await import(/* webpackChunkName: "day16" */ '../../day16/pkg/day16');
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day16();
