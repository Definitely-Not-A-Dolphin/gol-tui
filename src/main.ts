import { print } from "./text-utils.ts";

const { columns, rows } = Deno.consoleSize();

function main(): number {
  console.log(Deno.consoleSize());

  for (let i = 1; i <= columns; i++) {
    for (let j = 1; j <= rows; j++) {
      print("H");
    }
  }

  return 0;
}

if (import.meta.main) {
  Deno.exit(main());
}
