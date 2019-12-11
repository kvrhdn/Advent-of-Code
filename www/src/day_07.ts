async function load_day07() {
  let { part1, part2 } = await import("../../day07/pkg/day07");
  let { setupTemplate } = await import('./template');

  setupTemplate(part1, part2);
}

load_day07();
