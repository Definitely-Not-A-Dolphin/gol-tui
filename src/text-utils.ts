export const toUint8Array = (text: string) =>
  Uint8Array.from(
    Array.from(text).map((letter) => letter.charCodeAt(0)),
  );

export const print = (str: string) => {
  Deno.stdout.writeSync(toUint8Array(str));
};

export const codes = {
  chars: {
    upArrow: "\u001b[A",
    downArrow: "\u001b[B",
    leftArrow: "\u001b[C",
    rightArrow: "\u001b[D",
  },
  codes: {
    up: "\x1b[A", // \033[A
    down: "\n",
    begin: "\r",
    upBegin: "\x1b[F", // \033[F
  },
};
