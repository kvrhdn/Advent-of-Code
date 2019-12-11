async function load_day01() {
  let { part1, part2 } = await import("../../day01/pkg/day01");
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day01();
