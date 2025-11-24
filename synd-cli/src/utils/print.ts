const keyWidth = 30

export function print(key: string, value?: string | number) {
  value ? console.log(`${key}:`.padEnd(keyWidth) + value) : console.log(key)
}

export function printIndented(key: string, value: string | number, indent = 2) {
  const indentStr = " ".repeat(indent)
  console.log(`${indentStr}${key}:`.padEnd(keyWidth + indent) + value)
}

export function printSeparator(char = "=", width = 80) {
  console.log(char.repeat(width))
}

export function printSection(title: string, char = "=", width = 80) {
  printSeparator(char, width)
  console.log(`\n${title}\n`)
  printSeparator(char, width)
}

export function exitWithError(message: string, options?: { emoji?: boolean }) {
  const showEmoji = options?.emoji ?? true
  const prefix = showEmoji ? "❌ " : ""
  console.error(`${prefix}${message}`)
  process.exit(1)
}
