import { Temporal } from "@js-temporal/polyfill";

const SHORT_MONTHS = [
  "Jan",
  "Feb",
  "Mar",
  "Apr",
  "May",
  "Jun",
  "Jul",
  "Aug",
  "Sep",
  "Oct",
  "Nov",
  "Dec",
] as const;

const LONG_MONTHS = [
  "January",
  "February",
  "March",
  "April",
  "May",
  "June",
  "July",
  "August",
  "September",
  "October",
  "November",
  "December",
] as const;

const WEEKDAYS = [
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
  "Sunday",
] as const;

interface CalendarDate {
  day: number;
  month: number;
  year: number;
}

interface ClockTime {
  hour: number;
  minute: number;
}

function dateParts(date: Date | CalendarDate): CalendarDate {
  if (date instanceof Date) {
    return {
      day: date.getDate(),
      month: date.getMonth() + 1,
      year: date.getFullYear(),
    };
  }

  return date;
}

function timeParts(time: Date | ClockTime): ClockTime {
  if (time instanceof Date) {
    return {
      hour: time.getHours(),
      minute: time.getMinutes(),
    };
  }

  return time;
}

function ordinal(day: number): string {
  const remainder = day % 100;
  const suffix =
    remainder >= 11 && remainder <= 13
      ? "th"
      : (["th", "st", "nd", "rd", "th"] as const)[Math.min(day % 10, 4)];

  return `${day}${suffix}`;
}

export function formatDistance(meters: number): string {
  const km = Math.round(meters / 100) / 10;

  return `${km}km`;
}

export function formatVertical(meters: number): string {
  return `${Math.round(meters)}m`;
}

export function formatDuration(duration: Temporal.Duration): string {
  // Convert to total hours and minutes
  const totalHours = Math.floor(duration.total("hours"));
  const remainingMinutes = Math.floor(duration.total("minutes") % 60);

  return `${totalHours}h${remainingMinutes}m`;
}

export function formatDate(date: Date | CalendarDate): string {
  const { day, month, year } = dateParts(date);

  return `${ordinal(day)} ${SHORT_MONTHS[month - 1]} ${year}`;
}

export function formatTime(time: Date | ClockTime): string {
  const { hour, minute } = timeParts(time);

  return `${hour.toString().padStart(2, "0")}:${minute.toString().padStart(2, "0")}`;
}

export function formatDateTime(
  date: Date | (CalendarDate & ClockTime),
): string {
  return `${formatDate(date)}, ${formatTime(date)}`;
}

export function formatLongDate(
  date: CalendarDate & { dayOfWeek: number },
): string {
  const { day, month, year, dayOfWeek } = date;

  return `${WEEKDAYS[dayOfWeek - 1]}, ${ordinal(day)} ${LONG_MONTHS[month - 1]} ${year}`;
}

export function formatMonthYear(
  date: Pick<CalendarDate, "month" | "year">,
): string {
  const { month, year } = date;

  return `${SHORT_MONTHS[month - 1]} ${year}`;
}
