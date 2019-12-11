async function load_day09() {
  let { part1, part2 } = await import("../../day09/pkg/day09");
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day09();
