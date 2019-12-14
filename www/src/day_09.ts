async function load_day09() {
  let { part1, part2 } = await import(/* webpackChunkName: "day09" */ '../../day09/pkg/day09');
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day09();
