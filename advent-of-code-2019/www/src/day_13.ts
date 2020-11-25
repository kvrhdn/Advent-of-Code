async function load_day13() {
  let { part1, part2 } = await import(/* webpackChunkName: "day13" */ '../../day13/pkg/day13');
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day13();
