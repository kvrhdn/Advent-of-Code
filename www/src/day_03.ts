async function load_day03() {
  let { part1, part2 } = await import("../../day03/pkg/day03");
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day03();
