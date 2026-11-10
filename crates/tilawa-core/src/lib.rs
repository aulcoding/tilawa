//! Tilawa core: the 29-day plan and the rules for reading progress.
//!
//! Everything here is plain Rust with no browser code, so it can be tested
//! with `cargo test`. The page uses it through `tilawa-app` (Leptos).

use serde::{Deserialize, Serialize};

/// How many days the plan has.
pub const DAY_COUNT: usize = 29;
/// Pages in the Mushaf Madinah.
pub const LAST_PAGE: u16 = 604;

// ---------------------------------------------------------------------------
// The plan
// ---------------------------------------------------------------------------

/// A label shown on some day cards. The text itself lives in lang/*.js.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Tag {
    Boss,
    Pack,
    Half,
    End,
}

/// One day as written in the plan: which surahs, by number.
struct PlanDay {
    names: &'static [&'static str],
    first: u8,
    last: u8,
    tag: Option<Tag>,
}

const fn day(names: &'static [&'static str], first: u8, last: u8, tag: Option<Tag>) -> PlanDay {
    PlanDay { names, first, last, tag }
}

/// The 29-day plan, split by surah (never mid-surah).
const PLAN: [PlanDay; DAY_COUNT] = [
    day(&["Al-Fatihah", "Al-Baqarah"], 1, 2, Some(Tag::Boss)),
    day(&["Ali ‘Imran"], 3, 3, None),
    day(&["An-Nisa’"], 4, 4, None),
    day(&["Al-Ma’idah"], 5, 5, None),
    day(&["Al-An’am"], 6, 6, None),
    day(&["Al-A’raf"], 7, 7, None),
    day(&["Al-Anfal", "At-Taubah"], 8, 9, Some(Tag::Pack)),
    day(&["Yunus", "Hud"], 10, 11, None),
    day(&["Yusuf", "Ar-Ra’d", "Ibrahim"], 12, 14, None),
    day(&["Al-Hijr", "An-Nahl"], 15, 16, None),
    day(&["Al-Isra’", "Al-Kahf"], 17, 18, Some(Tag::Pack)),
    day(&["Maryam", "Taha"], 19, 20, None),
    day(&["Al-Anbiya’", "Al-Hajj"], 21, 22, None),
    day(&["Al-Mu’minun", "An-Nur"], 23, 24, None),
    day(&["Al-Furqan"], 25, 25, Some(Tag::Half)),
    day(&["Asy-Syu’ara’", "An-Naml"], 26, 27, None),
    day(&["Al-Qasas", "Al-‘Ankabut"], 28, 29, None),
    day(&["Ar-Rum", "Luqman", "As-Sajdah"], 30, 32, None),
    day(&["Al-Ahzab", "Saba’"], 33, 34, None),
    day(&["Fatir", "Yasin", "As-Saffat"], 35, 37, None),
    day(&["Sad", "Az-Zumar"], 38, 39, None),
    day(&["Gafir", "Fussilat"], 40, 41, None),
    day(&["Asy-Syura", "Az-Zukhruf", "Ad-Dukhan"], 42, 44, None),
    day(&["Al-Jasiyah", "Al-Ahqaf", "Muhammad", "Al-Fath"], 45, 48, None),
    day(&["Al-Hujurat – An-Najm"], 49, 53, None),
    day(&["Al-Qamar – Al-Hadid"], 54, 57, None),
    day(&["Al-Mujadalah – At-Tahrim"], 58, 66, None),
    day(&["Al-Mulk – Al-Mursalat"], 67, 77, None),
    day(&["An-Naba’ – An-Nas"], 78, 114, Some(Tag::End)),
];

/// One entry of quran.json.
#[derive(Debug, Deserialize)]
struct Surah {
    no: u8,
    start_page: u16,
}

/// quran.json is baked into the .wasm at compile time, so the browser never fetches it.
const QURAN_JSON: &str = include_str!("../../../quran.json");

/// A day with its page range worked out. This is what the page renders.
/// Field names match what the existing JavaScript already uses (s, r, tag, pg).
#[derive(Debug, Clone, Serialize)]
pub struct Day {
    #[serde(rename = "s")]
    pub surahs: Vec<String>,
    /// First and last surah number.
    #[serde(rename = "r")]
    pub surah_range: [u8; 2],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
    /// First and last page of the day.
    #[serde(rename = "pg")]
    pub pages: [u16; 2],
}

impl Day {
    /// How many pages this day has.
    pub fn total(&self) -> u16 {
        self.pages[1] - self.pages[0] + 1
    }
}

/// Builds the 29 days with page ranges from quran.json.
///
/// A day runs from the start page of its first surah to the page before the
/// next day's first surah. (A page shared by two surahs counts for the day that
/// starts it.) The last day runs to page 604.
pub fn build_days() -> Vec<Day> {
    let surahs: Vec<Surah> = serde_json::from_str(QURAN_JSON).expect("quran.json is valid");
    let start_of = |no: u8| -> u16 {
        surahs
            .iter()
            .find(|s| s.no == no)
            .map(|s| s.start_page)
            .unwrap_or_else(|| panic!("surah {no} is missing from quran.json"))
    };

    PLAN.iter()
        .map(|d| {
            let first_page = start_of(d.first);
            let last_page = if d.last == 114 { LAST_PAGE } else { start_of(d.last + 1) - 1 };
            Day {
                surahs: d.names.iter().map(|n| n.to_string()).collect(),
                surah_range: [d.first, d.last],
                tag: d.tag,
                pages: [first_page, last_page],
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Progress
// ---------------------------------------------------------------------------

/// What happened after a tap. The page uses this to decide what to show.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Outcome {
    /// A page was added; the day is not finished yet.
    Added,
    /// A page was added and that finished the day.
    Finished,
    /// A page was taken away.
    Removed,
    /// Nothing changed (e.g. "+" on a full day, "−" on an empty one).
    Noop,
    /// The day is ahead of schedule. `next` is the day to read first.
    Locked { next: u8 },
    /// Pages must be taken back from the end. `last` is the day to undo first.
    UndoFromEnd { last: u8 },
}

/// Pages read per day. Days are read in order: nothing counts after the first unfinished day.
#[derive(Debug, Clone)]
pub struct Progress {
    days: Vec<Day>,
    /// pages[i] = pages read on day i + 1
    pages: [u16; DAY_COUNT],
}

impl Progress {
    pub fn new(days: Vec<Day>) -> Self {
        assert_eq!(days.len(), DAY_COUNT);
        Progress { days, pages: [0; DAY_COUNT] }
    }

    /// Restores saved progress.
    ///
    /// * `saved_pages`: the current format, a JSON array of pages per day.
    /// * `old_done`: the older format, a JSON array of finished day numbers.
    ///   Used only when there are no saved pages; a finished day counts as fully read.
    ///
    /// Bad or missing data is ignored rather than failing, and the result is
    /// always valid (in range, in order).
    pub fn restore(days: Vec<Day>, saved_pages: Option<&str>, old_done: Option<&str>) -> Self {
        let mut p = Progress::new(days);
        if let Some(arr) = saved_pages.and_then(|s| serde_json::from_str::<Vec<i64>>(s).ok()) {
            for (i, v) in arr.into_iter().take(DAY_COUNT).enumerate() {
                p.pages[i] = v.clamp(0, p.days[i].total() as i64) as u16;
            }
        } else if let Some(done) = old_done.and_then(|s| serde_json::from_str::<Vec<i64>>(s).ok()) {
            // Only the unbroken run from Day 1 counts
            for d in 1..=DAY_COUNT {
                if !done.contains(&(d as i64)) {
                    break;
                }
                p.pages[d - 1] = p.days[d - 1].total();
            }
        }
        p.enforce_order();
        p
    }

    /// Clears anything recorded after the first unfinished day.
    fn enforce_order(&mut self) {
        if let Some(open) = (0..DAY_COUNT).find(|&i| self.pages[i] < self.days[i].total()) {
            for v in &mut self.pages[open + 1..] {
                *v = 0;
            }
        }
    }

    pub fn days(&self) -> &[Day] {
        &self.days
    }

    pub fn pages(&self) -> &[u16; DAY_COUNT] {
        &self.pages
    }

    /// Pages read on a day (1-based).
    pub fn pages_on(&self, day: u8) -> u16 {
        self.pages[day as usize - 1]
    }

    pub fn is_finished(&self, day: u8) -> bool {
        let i = day as usize - 1;
        self.pages[i] >= self.days[i].total()
    }

    /// How many days are finished (always the first N days).
    pub fn finished_count(&self) -> u8 {
        (1..=DAY_COUNT as u8).take_while(|&d| self.is_finished(d)).count() as u8
    }

    /// The day being read now, or None after khatam.
    pub fn current_day(&self) -> Option<u8> {
        let n = self.finished_count();
        (n < DAY_COUNT as u8).then_some(n + 1)
    }

    /// The last day that has any pages: the only one "−" may touch.
    fn last_with_pages(&self) -> Option<u8> {
        (1..=DAY_COUNT as u8).rev().find(|&d| self.pages_on(d) > 0)
    }

    pub fn can_add(&self, day: u8) -> bool {
        self.current_day() == Some(day) && !self.is_finished(day)
    }

    pub fn can_remove(&self, day: u8) -> bool {
        self.last_with_pages() == Some(day)
    }

    /// One more page read on `day` (a tap on the card or "+").
    pub fn add_page(&mut self, day: u8) -> Outcome {
        let Some(cur) = self.current_day() else { return Outcome::Noop };
        if day > cur {
            return Outcome::Locked { next: cur };
        }
        if day < cur {
            return Outcome::Noop; // already finished
        }
        let i = day as usize - 1;
        self.pages[i] += 1;
        if self.is_finished(day) { Outcome::Finished } else { Outcome::Added }
    }

    /// One page taken back on `day` ("−").
    pub fn remove_page(&mut self, day: u8) -> Outcome {
        if let Some(cur) = self.current_day() {
            if day > cur {
                return Outcome::Locked { next: cur };
            }
        }
        match self.last_with_pages() {
            None => Outcome::Noop,
            Some(last) if last == day => {
                self.pages[day as usize - 1] -= 1;
                Outcome::Removed
            }
            // "−" on the day in progress when it is still empty: nothing to remove
            Some(_) if self.pages_on(day) == 0 => Outcome::Noop,
            Some(last) => Outcome::UndoFromEnd { last },
        }
    }

    pub fn reset(&mut self) {
        self.pages = [0; DAY_COUNT];
    }

    /// JSON for localStorage (same format the JavaScript version wrote).
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.pages.to_vec()).expect("numbers always serialise")
    }

    /// Finished day numbers, for the older storage key.
    pub fn finished_days_json(&self) -> String {
        let done: Vec<u8> = (1..=self.finished_count()).collect();
        serde_json::to_string(&done).expect("numbers always serialise")
    }
}

// ---------------------------------------------------------------------------
// Tests: run with `cargo test`
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> Progress {
        Progress::new(build_days())
    }

    /// Fills a day completely.
    fn finish(p: &mut Progress, day: u8) {
        while !p.is_finished(day) {
            p.add_page(day);
        }
    }

    #[test]
    fn plan_covers_all_604_pages_without_gaps() {
        let days = build_days();
        assert_eq!(days.len(), 29);
        assert_eq!(days[0].pages[0], 1);
        assert_eq!(days[28].pages[1], 604);
        for pair in days.windows(2) {
            assert_eq!(pair[0].pages[1] + 1, pair[1].pages[0], "gap or overlap between days");
        }
        let total: u16 = days.iter().map(Day::total).sum();
        assert_eq!(total, 604);
    }

    #[test]
    fn day_one_is_49_pages_and_day_fifteen_is_8() {
        let days = build_days();
        assert_eq!(days[0].total(), 49);
        assert_eq!(days[14].total(), 8);
    }

    #[test]
    fn tapping_adds_one_page() {
        let mut p = fresh();
        assert_eq!(p.add_page(1), Outcome::Added);
        assert_eq!(p.pages_on(1), 1);
    }

    #[test]
    fn last_page_finishes_the_day() {
        let mut p = fresh();
        for _ in 0..48 {
            p.add_page(1);
        }
        assert_eq!(p.add_page(1), Outcome::Finished);
        assert!(p.is_finished(1));
        assert_eq!(p.finished_count(), 1);
        assert_eq!(p.current_day(), Some(2));
    }

    #[test]
    fn cannot_skip_ahead() {
        let mut p = fresh();
        assert_eq!(p.add_page(2), Outcome::Locked { next: 1 });
        assert_eq!(p.pages_on(2), 0);
    }

    #[test]
    fn plus_on_a_full_day_does_nothing() {
        let mut p = fresh();
        finish(&mut p, 1);
        assert_eq!(p.add_page(1), Outcome::Noop);
        assert_eq!(p.pages_on(1), 49);
    }

    #[test]
    fn minus_on_an_empty_day_does_nothing() {
        let mut p = fresh();
        assert_eq!(p.remove_page(1), Outcome::Noop);
    }

    #[test]
    fn undo_must_start_from_the_end() {
        let mut p = fresh();
        finish(&mut p, 1);
        p.add_page(2);
        p.add_page(2);
        assert_eq!(p.remove_page(1), Outcome::UndoFromEnd { last: 2 });
        assert_eq!(p.pages_on(1), 49);
        assert_eq!(p.remove_page(2), Outcome::Removed);
        assert_eq!(p.remove_page(2), Outcome::Removed);
        // Day 2 is empty again, so the last finished day can now be undone
        assert_eq!(p.remove_page(1), Outcome::Removed);
        assert_eq!(p.pages_on(1), 48);
        assert_eq!(p.current_day(), Some(1));
    }

    #[test]
    fn restore_reads_saved_pages_and_clamps_bad_values() {
        let p = Progress::restore(build_days(), Some("[49, 999, -3]"), None);
        assert_eq!(p.pages_on(1), 49);
        assert_eq!(p.pages_on(2), 27); // clamped to the day's total
        assert_eq!(p.pages_on(3), 0); // negative becomes 0
    }

    #[test]
    fn restore_drops_progress_after_an_unfinished_day() {
        let p = Progress::restore(build_days(), Some("[10, 5, 3]"), None);
        assert_eq!(p.pages_on(1), 10);
        assert_eq!(p.pages_on(2), 0);
        assert_eq!(p.pages_on(3), 0);
    }

    #[test]
    fn restore_converts_the_old_finished_days_format() {
        // Days 1, 2 and 5 were ticked; only the unbroken run 1–2 counts
        let p = Progress::restore(build_days(), None, Some("[1, 2, 5]"));
        assert_eq!(p.finished_count(), 2);
        assert_eq!(p.pages_on(5), 0);
    }

    #[test]
    fn restore_survives_garbage() {
        let p = Progress::restore(build_days(), Some("not json"), Some("{}"));
        assert_eq!(p.finished_count(), 0);
    }

    #[test]
    fn json_round_trip() {
        let mut p = fresh();
        finish(&mut p, 1);
        p.add_page(2);
        let again = Progress::restore(build_days(), Some(&p.to_json()), None);
        assert_eq!(again.pages(), p.pages());
        assert_eq!(p.finished_days_json(), "[1]");
    }

    #[test]
    fn khatam_after_all_days() {
        let mut p = fresh();
        for d in 1..=29 {
            finish(&mut p, d);
        }
        assert_eq!(p.finished_count(), 29);
        assert_eq!(p.current_day(), None);
        assert_eq!(p.add_page(29), Outcome::Noop);
    }
}
