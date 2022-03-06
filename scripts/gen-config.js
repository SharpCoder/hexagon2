/*
    This script is designed to generate the config file
    which is uploaded to an s3 bucket and pulled down occasionally
    so the hexagon wall knows if a specific day is special. And,
    if it is special, what theme to play that day.
*/

const filer_themes = [
    "R2D2",
    "Jupiter",
    "Mars",
    "Duna",
    "Starfleet",
    "Rainbow",
    "Dinosaur",
    "Medbay",
    "RetroFuturistic",
];

const important_events = [
    {
        note: 'Sams Birthday',
        shader: 'Birthday',
        origin: new Date("04-07-2022"),
    },
    {
        note: 'My Birthday',
        shader: 'Birthday',
        origin: new Date("04-28-2022"),
    },
    {
        note: '4th of July',
        shader: 'Independence',
        origin: new Date("07-04-2022"),
    },
    {
        note: 'Christmas',
        shader: 'Xmas',
        origin: new Date("12-25-2022"),
        range_start: new Date("12-01-2022"),
        range_end: new Date("12-31-2022"),
    },
    {
        note: 'Halloween',
        shader: 'Halloween',
        origin: new Date("10-31-2022"),
        range_start: new Date("10-01-2022"),
        range_end: new Date("10-31-2022"),
    },
    {
        note: 'Valendines Day',
        shader: 'Valentines',
        origin: new Date("02-14-2022"),
    },
];

const one_off_events = [
    { shader: 'Lunar', origin: new Date("01-22-2023") },
    { shader: 'Lunar', origin: new Date("02-10-2024") },
    { shader: 'Lunar', origin: new Date("01-29-2025") },
    { shader: 'Lunar', origin: new Date("02-17-2026") },
    { shader: 'Lunar', origin: new Date("02-06-2027") },
    { shader: 'Lunar', origin: new Date("01-26-2028") },
    { shader: 'Lunar', origin: new Date("02-13-2029") },
    { shader: 'Lunar', origin: new Date("02-03-2030") },
    { shader: 'Thanksgiving', origin: new Date("11-24-2022") },
    { shader: 'Thanksgiving', origin: new Date("11-23-2023") },
    { shader: 'Thanksgiving', origin: new Date("11-28-2024") },
    { shader: 'Thanksgiving', origin: new Date("11-27-2025") },
    { shader: 'Thanksgiving', origin: new Date("11-26-2026") },
    { shader: 'Thanksgiving', origin: new Date("11-25-2027") },
    { shader: 'Thanksgiving', origin: new Date("11-23-2028") },
    { shader: 'Thanksgiving', origin: new Date("11-22-2029") },
    { shader: 'Thanksgiving', origin: new Date("11-28-2030") },
];