import assert from "node:assert/strict";
import { test } from "node:test";
import {
  formatDate,
  formatDateTime,
  formatLongDate,
  formatMonthYear,
  formatTime,
} from "../app/services/format";

test("formats dates with ordinal days and 24-hour times", () => {
  const cases: ReadonlyArray<readonly [number, string]> = [
    [1, "1st"],
    [2, "2nd"],
    [3, "3rd"],
    [4, "4th"],
    [11, "11th"],
    [12, "12th"],
    [13, "13th"],
    [21, "21st"],
    [22, "22nd"],
    [23, "23rd"],
    [31, "31st"],
  ];

  for (const [day, expectedDay] of cases) {
    const date = new Date(2026, 0, day, 14, 27);

    assert.equal(formatDateTime(date), `${expectedDay} Jan 2026, 14:27`);
  }
});

test("formats every date shape without ambiguous numeric dates", () => {
  const date = { day: 7, month: 11, year: 2026 };
  const dateTime = { ...date, hour: 14, minute: 27 };

  assert.equal(formatDate(date), "7th Nov 2026");
  assert.equal(formatDateTime(dateTime), "7th Nov 2026, 14:27");
  assert.equal(
    formatLongDate({ ...date, dayOfWeek: 6 }),
    "Saturday, 7th November 2026",
  );
  assert.equal(formatMonthYear(date), "Nov 2026");
  assert.equal(formatTime(dateTime), "14:27");
});
