# Project 086 – Time Zone Converter CLI (with chrono-tz)

## What I Built
A CLI tool that converts datetime strings between time zones using the chrono and chrono-tz crates. Learn't to parse strings, convert them across zones, and handle user input for global time conversions.

## What I Learned


## Notes
### How to Run the Application:
```
cargo run

```

### Example Interaction

```
# Lagos to New York:

🕒 Enter datetime: 2025-06-15 14:00
🌐 Source timezone: WAT
🌐 Target timezone: EST

✅ 2025-06-15 14:00:00 in WAT (Africa/Lagos) → 2025-06-15 09:00:00 in EST (America/New_York)


# Nairobi to London:

🕒 Enter datetime: 2025-12-01 09:30
🌐 Source timezone: EAT  
🌐 Target timezone: GMT

✅ 2025-12-01 09:30:00 in EAT (Africa/Nairobi) → 2025-12-01 06:30:00 in GMT (Europe/London)

# Johannesburg to Dubai:

🕒 Enter datetime: now
🌐 Source timezone: SAST
🌐 Target timezone: GST

✅ 2024-01-15 13:45:00 in SAST (Africa/Johannesburg) → 2024-01-15 15:45:00 in GST (Asia/Dubai)
```











