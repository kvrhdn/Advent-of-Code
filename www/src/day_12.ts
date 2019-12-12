async function load_day12() {
  let { part1, part2 } = await import("../../day12/pkg/day12");
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day12();
