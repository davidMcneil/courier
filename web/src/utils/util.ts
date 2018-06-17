export function assert(condition: boolean, message: string = "Assertion failed") {
  if (!condition) {
    if (typeof Error !== undefined) {
      throw new Error(message);
    }
    throw message; // Fallback
  }
}

export function assertNotUndefined(value: any, message: string = "Undefined!") {
  assert(value !== undefined, message);
}

export function abbreviateNumber(value: number, decimalDigits: number = 1): string {
  const digits = Math.floor(value).toString().length;
  if (digits <= 3) {
    return value.toFixed(decimalDigits);
  } else if (digits >= 4 && digits <= 6) {
    return (value / 1000).toFixed(decimalDigits) + "k";
  } else if (digits >= 7 && digits <= 9) {
    return (value / 1000000).toFixed(decimalDigits) + "m";
  } else if (digits >= 10 && digits <= 12) {
    return (value / 1000000000).toFixed(decimalDigits) + "b";
  } else if (digits >= 13) {
    return (value / 1000000000000).toFixed(decimalDigits) + "t";
  } else {
    return value.toFixed(decimalDigits);
  }
}

export function numberAsPercentage(value: number, decimalDigits: number = 1): string {
  return (value * 100).toFixed(decimalDigits) + "%";
}

export function numberAsSize(bytes: number, decimalDigits: number = 1): string {
  const digits = Math.floor(bytes).toString().length;
  if (digits <= 3) {
    return bytes.toFixed(decimalDigits) + "B";
  } else if (digits >= 4 && digits <= 6) {
    return (bytes / 1000).toFixed(decimalDigits) + "KB";
  } else if (digits >= 7 && digits <= 9) {
    return (bytes / 1000000).toFixed(decimalDigits) + "MB";
  } else if (digits >= 10 && digits <= 12) {
    return (bytes / 1000000000).toFixed(decimalDigits) + "GB";
  } else if (digits >= 13) {
    return (bytes / 1000000000000).toFixed(decimalDigits) + "TB";
  } else {
    return bytes.toFixed(decimalDigits);
  }
}

export function numberAsTime(seconds: number): [number, number, number] {
  const sInM = 60;
  const sInH = 60 * sInM;
  const sInD = 24 * sInH;
  const days = Math.floor(seconds / sInD);
  seconds = seconds - days * sInD;
  const hours = Math.floor(seconds / sInH);
  seconds = seconds - hours * sInH;
  const minutes = Math.floor(seconds / sInM);
  return [days, hours, minutes];
}

export function numberAsTimeStr(seconds: number): string {
  const [d, h, m] = numberAsTime(seconds);
  return `${d}d ${h}h ${m}m`;
}

export function str2uint(s: string, def: number = 0): number {
  let n = parseInt(s, 10);
  if (isNaN(n) || n < 0) {
    n = 0;
  }
  return n;
}

export function str2number(s: string): number | null {
  const interval = parseFloat(s);
  if (!isNumber(interval)) {
    return null;
  }
  return interval;
}

export function isObject(blob: any): blob is any {
  return blob === Object(blob);
}

export function isUndefined(blob: any): blob is undefined {
  return blob === undefined;
}

export function isArray(blob: any): blob is any[] {
  return Array.isArray(blob);
}

export function isString(blob: any): blob is string {
  return typeof blob === "string" || blob instanceof String;
}

export function isNumber(blob: any): blob is number {
  return !isNaN(blob);
}

export function isJson(str: string): [boolean, any] {
  try {
    return [true, JSON.parse(str)];
  } catch (e) {
    return [false, null];
  }
}

export function rootUrl(): string {
  return "http://0.0.0.0:3140";
}

export function apiPrefix(): string {
  return "api/v0";
}

export function metricsUrl(): string {
  return `${rootUrl()}/${apiPrefix()}/metrics`;
}

export function topicsUrl(topic: string = ""): string {
  return `${rootUrl()}/${apiPrefix()}/topics/${topic}`;
}

export function subscriptionsUrl(subscription: string = ""): string {
  return `${rootUrl()}/${apiPrefix()}/subscriptions/${subscription}`;
}

export function publishUrl(topic: string): string {
  return `${topicsUrl(topic)}/publish`;
}

export function pullUrl(subscription: string): string {
  return `${subscriptionsUrl(subscription)}/pull`;
}

export function ackUrl(subscription: string): string {
  return `${subscriptionsUrl(subscription)}/ack`;
}

export function logError(message: string, error: any) {
  // tslint:disable-next-line:no-console
  console.error(message, error.stack);
}

export function fetchError2message(error: Error | Response): string {
  if (error instanceof Response) {
    return error.status.toString();
  } else {
    return error.message;
  }
}

export const HEADERS = new Headers();
HEADERS.append("Content-Type", "application/json");
