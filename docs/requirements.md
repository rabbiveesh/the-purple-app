# Pretzel: product requirements

What the app is for, as decided with the user (October 2026). Where the code and this file
disagree, this file wins until it is updated.

## Goal

The user knows the pelvic floor routine her physical therapist prescribed.
What stops her is forgetting, not knowing how long 10 seconds is without a stopwatch, and the
tedium. The app walks her through the routine twice a day, times everything, and makes it fun
enough that it actually happens.

## Disguise

- The app is called **Pretzel**, with a pretzel icon. Nobody glancing at her phone, for example in
  a staff meeting, should be able to tell what it is for.
- The words "kegel" and "pelvic" never appear anywhere visible: app name, icon label, screen
  titles, notifications, or the voice.
- Notifications say something like "Pretzel time!".
- The leak questions sit behind a tap (the weekly check-in), never on a screen she lands on.

## The routine

One session, twice a day:

| Exercise | One rep | Reps | Between reps |
| --- | --- | --- | --- |
| 1. Slow squeezes | Squeeze up over 5 s, release over 5 s | 10 | 3 s rest |
| 2. Long holds | Hold 10 s, then release for 3 s | 10 | (the release) |
| 3. Quick squeezes | 10 short squeezes, one per second | 10 | 3 s rest |

About 6.7 minutes. The routine is data in the core, so it can change later (the physical
therapist may make it harder over time).

## A session

1. "Hey, ready to start?" She picks a coach, and that tap starts the session:
   - **Silent**: screen only. No sound and no vibration. For meetings; looking at the screen is fine.
   - **Voice + screen**: the coach counts out loud.
   - **Voice only**: for the car, next to Waze.
2. Each exercise is announced with "Exercise N starts in 5 seconds" and then simply starts. There is
   no "yes" to answer (voice "yes" is postponed).
3. On screen: the phase in big letters (Squeeze, Release, Rest), the seconds counted up as the coach
   counts ("squeeze, 2, 3, 4, 5"), and a bar that rises while squeezing and falls while releasing.
4. Between exercises, a short "Done". Nothing gushing.
5. A big pause button is always there. When paused, she can carry on or stop for now.
6. The screen stays awake during a session.
7. At the end: a short survey, then something dances on screen (a pretzel, a chubby baby, or a
   random surprise) with confetti.

## Progress and fun

- A calendar with a morning and an afternoon circle per day, filled purple when done.
- She can tick off a session she did without the app by tapping its circle, and untick it.
  Sessions done with the coach can't be unticked by accident.
- Nothing bad happens on a missed day.
- Streaks are cheered from two days in a row. A streak counts days with at least one session.
- Pretzel points: 10 per session, 5 extra for doing both in a day. They fill a pretzel jar of 100;
  a full jar starts a new one.
- Pretzelometer rotomies: one per squeeze, 120 per full session.
- Bright, fun colours, mostly purple.

## Survey

Based on how pelvic floor training research measures progress (leaks, how much they bother you,
and the person's own sense of improvement), in our own words rather than a licensed questionnaire.

| When | Question | Answers |
| --- | --- | --- |
| After each session | How hard was that? | 1 to 10 |
| After each session | Could you hold the full 10 seconds? | All / most / some / few |
| After each session | Were the last squeezes as strong as the first? | Yes / a bit weaker / much weaker |
| After each session (optional) | Could you fully relax in between? | Yes / not really |
| Weekly | Any leaks this week? | None / once or twice / a few times / most days |
| Weekly | When? | Cough or sneeze / laughing / jumping or running / lifting the kids / sudden urge |
| Weekly | How much did it bother you? | 0 to 10 |
| Weekly (optional) | Any heaviness or dragging feeling? | Never / sometimes / often |
| Every 4 weeks, from week 4 | Compared with when you started, how are things now? | Much better ... much worse (7 steps) |

The after-session survey can be skipped. The weekly and monthly check-ins appear on the home
screen when due.

## Reminders

Around 9:00 and around 16:00, as push notifications. A web app can't schedule its own
notifications, so a small Cloudflare Worker on a cron sends them. Not built yet.

## Phone

Android, Chrome, installed to the home screen. Data lives only on the phone, so there is a backup
and restore screen.

## Open

- Recorded voice clips instead of the phone's built-in voice (decided, not done yet). Whose voice.
- How the coach's voice and Spotify get along in practice (dip or pause): test on the phone.
- Whether voice-only mode should play the session as one audio track so it keeps going behind Waze.
- What a full jar earns, and what else streaks unlock.
- Whether a partly finished session should count (today it doesn't).
- Whether a missed reminder should nudge again later.
- Survey trends: answers are saved but not yet shown as charts.
