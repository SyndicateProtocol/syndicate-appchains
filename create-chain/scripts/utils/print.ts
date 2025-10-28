const keyWidth = 22
export function print(key: string, value?: string | number) {
  value ? console.log(`${key}:`.padEnd(keyWidth) + value) : console.log(key)
}
