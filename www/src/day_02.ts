async function load_day02() {
  let { part1, part2 } = await import(/* webpackChunkName: "day02" */ '../../day02/pkg/day02');
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day02();
