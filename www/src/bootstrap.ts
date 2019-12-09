let days = [
  '01', '02', '03', '04', '05',
  '06', '07', '08', '09', '10',
];

for (const d of days) {
  import(`./index_day${d}.ts`)
    .catch(console.error);
}
