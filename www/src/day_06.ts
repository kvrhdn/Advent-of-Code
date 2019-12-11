async function load_day06() {
  let { part1, part2 } = await import("../../day06/pkg/day06");
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day06();
