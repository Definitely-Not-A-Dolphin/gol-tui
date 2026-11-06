import { print, toPos } from "./text-utils.ts";
import { arrayNElements } from "./utils.ts";

Deno.stdin.setRaw(true);

let { columns, rows } = Deno.consoleSize();
rows = rows - 20;

let [currentColumn, currentRow] = [0, 0];
const decoder = new TextDecoder();
const field = arrayNElements(rows).map(() =>
  arrayNElements(columns).map(() => false)
);

field[0][0] = true;

function printField(field: boolean[][]): void {
  console.clear();
  toPos(0, 0);

  let fieldString = "";

  for (const row of field) {
    for (const element of row) {
      fieldString += element ? "H" : " ";
    }

    fieldString = fieldString.trimEnd();
    fieldString += "\n";
  }

  print(fieldString);
}

async function main(): Promise<number> {
  printField(field);

  for await (const chunk of Deno.stdin.readable) {
    const text = decoder.decode(chunk);

    switch (text) {
      case "q":
        return 0;
      case "w":
        if (0 < currentRow) currentRow--;
        break;
      case "s":
        if (currentRow < rows - 1) currentRow++;
        break;
      case "a":
        if (0 < currentColumn) currentColumn--;
        break;
      case "d":
        if (currentColumn < columns - 1) currentColumn++;
        break;
    }

    field[currentRow][currentColumn] = true;

    printField(field);
  }

  print("\x1b[0m");
  console.clear();

  return 0;
}

if (import.meta.main) {
  Deno.exit(await main());
}
