async function load_day10() {
  let { part1, part2 } = await import("../../day10/pkg/day10");
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day10();
