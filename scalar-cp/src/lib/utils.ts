export function wait(ms: number, value: any) {
  return new Promise((resolve) => setTimeout(resolve, ms, value));
}
