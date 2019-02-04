import * as process from "process";

export const ICON =
  // tslint:disable-next-line:max-line-length
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAQAAADZc7J/AAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAAmJLR0QAAKqNIzIAAAAJcEhZcwAADdcAAA3XAUIom3gAAAAHdElNRQfiCQ4PCxcR7B5aAAACGUlEQVRIx+3TXWiOcRgG8B/GiJphmeVjyHJAjNnJDkTKR7PZwU5Q3iimKGTLgaSJ2hwpRzjAcLDZgchkRVE7YMk0HxuG5WubadObTanXwd49PW975Vy7ju7nuq7u//1//tfNKP5LLPPToEE9pofY6wYMGPBO6b9bzHXeXbMSuP3emi9bhbZEc0qSBp2OeW29vhDXJUuehQ67mmgek3SGiV7I8jSBy9dmg6la/jbBcnlyfXbfY8f1yrAnZN5hlvFKvFdqmieaE9tnqNfntkoXvBH121JVbgT6TL02uiSmyRmVbutTL2NYXqTLFenBTK16nTfDD6viXK0aR7V7bnecSXdFl0UwTpOq0DRHPJPptXInNYBi3U7oNM8WnSYEzipNxlGmVWpA5ohahSV+yNSvQJpPtvpmJWi2L/CmalVGnYOhN3ngdLx+br1K95xzU65PcXaTzyYF/oPq6FAQEGV+2SsiIuKeWw6I6TfbYTWBpyl0YIGOFGmhwMTcVxyvs6VIEXXKR5m+hDK5OKj7pNEokjRMDfaYYtACbNOY1BPRONYjm5NIk612xy7fRfFYfsJqDWOzR2ToVjRCqlEv01OvVICLro3wFOkeClOJHoUhIVW1D0GwhpCmU3XouSnUo2T4o9hXtXbKtcYh7R7KGXHeHI1eKrdWrp1qfR363cPbOMM+eVbo16LBZbEkNx5ju3VWxpfprG9G8b/gDwJ2mQ7P17mEAAAAJXRFWHRkYXRlOmNyZWF0ZQAyMDE4LTA5LTE0VDE1OjExOjIzKzAyOjAwOUZQJgAAACV0RVh0ZGF0ZTptb2RpZnkAMjAxOC0wOS0xNFQxNToxMToyMyswMjowMEgb6JoAAAAZdEVYdFNvZnR3YXJlAHd3dy5pbmtzY2FwZS5vcmeb7jwaAAAAAElFTkSuQmCC";

export enum NotificationType {
  Success,
  Failure,
}

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

export function abbreviateInteger(value: number, decimalDigits: number = 1): string {
  const digits = Math.floor(value).toString().length;
  if (digits <= 3) {
    return value.toFixed(0);
  } else if (digits >= 4 && digits <= 6) {
    return (value / 1000).toFixed(decimalDigits) + "k";
  } else if (digits >= 7 && digits <= 9) {
    return (value / 1000000).toFixed(decimalDigits) + "m";
  } else if (digits >= 10 && digits <= 12) {
    return (value / 1000000000).toFixed(decimalDigits) + "b";
  } else if (digits >= 13) {
    return (value / 1000000000000).toFixed(decimalDigits) + "t";
  } else {
    return value.toFixed(0);
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

export const BASE_PATH = process.env.NODE_ENV === "development" ? "/" : "/ui";

const ROOT_URL =
  process.env.NODE_ENV === "development" ? "http://0.0.0.0:3140" : window.location.origin;

const API_PREFIX = "api/v1";

const FULL_URL = `${ROOT_URL}/${API_PREFIX}`;

export const METRICS_URL = `${FULL_URL}/metrics`;

export function topicsUrl(topic: string = ""): string {
  return `${FULL_URL}/topics/${topic}`;
}

export function subscriptionsUrl(subscription: string = ""): string {
  return `${FULL_URL}/subscriptions/${subscription}`;
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

export function logError(...messages: any[]) {
  // tslint:disable-next-line:no-console
  console.error(...messages);
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
