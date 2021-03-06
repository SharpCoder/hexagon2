/*
    This script is designed to generate the config file
    which is uploaded to an s3 bucket and pulled down occasionally
    so the hexagon wall knows if a specific day is special. And,
    if it is special, what theme to play that day.
*/
const SHADER_TYPE = 'rule';
const TIME_TYPE = 'time';
const DELAY_TYPE = 'delay';

const S_TO_NANO = 1000000000;
const H_TO_S = 60 * 60;
const DELAY_TTL = 1 * H_TO_S * S_TO_NANO;

const YEARS_FROM_2022 = 3 + (new Date().getFullYear() - 2022);
const filler_themes = [
    { shader: 'R2D2', priority: 6 },
    { shader: 'RetroFuturistic', priority: 6 },
    { shader: 'Pirate', priority: 5 },
    { shader: 'Starfleet', priority: 4 },
    { shader: 'DoctorWho', priority: 4 },
    { shader: 'Mars', priority: 4 },
    { shader: 'Duna', priority: 4 },
    { shader: 'Jupiter', priority: 4 },
    { shader: 'Rainbow', priority: 3 },
    { shader: 'Shire', priority: 1 },
    { shader: 'Pride', priority: 1 },
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
        note: 'Pride',
        shader: 'Pride',
        origin: new Date("06-28-2022"),
    },
    {
        note: 'Pokemon Day',
        shader: 'Pokemon',
        origin: new Date("02-27-2022"),
    },
    {
        note: 'Picard Day',
        shader: 'Starfleet',
        origin: new Date("06-16-2022"),
    },
    {
        note: 'Doctor Who Day',
        shader: 'DoctorWho',
        origin: new Date("11-23-2022"),
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
    {
        note: 'Star Wars Day',
        shader: 'R2D2',
        origin: new Date('05-04-2022'),
    },
    {
        note: 'Additional entry for Lunar New Year',
        shader: 'Lunar',
        origin: new Date("01-31-2022"),
        range_start: new Date("01-15-2022"),
        range_end: new Date("02-15-2022"),
    }
];

const one_off_events = [
    { shader: 'R2D2', origin: new Date('03-07-2022') },
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
    { shader: 'Diwali', origin: new Date("10-24-2022") },
    { shader: 'Diwali', origin: new Date("11-12-2023") },
    { shader: 'Diwali', origin: new Date("11-01-2024") },
    { shader: 'Diwali', origin: new Date("10-21-2025") },
    { shader: 'Diwali', origin: new Date("11-08-2026") },
    { shader: 'Diwali', origin: new Date("10-29-2027") },
    { shader: 'Diwali', origin: new Date("10-17-2028") },
    { shader: 'Diwali', origin: new Date("11-05-2029") },
    { shader: 'Diwali', origin: new Date("10-26-2030") },
];

// Take a bunch of attributes and returns an encoded line item
function encode(type, shader, start_date, end_date, priority) {
    // All entries have a shader and an origin
    return `${type};${start_date.getTime() / 1000};${end_date.getTime() / 1000};${shader};${priority};`;
}

exports.generate = () => {
    let lines = [];

    // Generate current time
    lines.push(`${TIME_TYPE};${new Date().getTime()}`);
    lines.push(`${DELAY_TYPE};${DELAY_TTL}`);
    
    // Generate the filler content
    let filler_start_date = new Date("01-01-2022");
    let filler_end_date = new Date("01-01-2222");
    for (const filler of filler_themes) {
        lines.push(
            encode(SHADER_TYPE, filler.shader, filler_start_date, filler_end_date, filler.priority)
        );
    }

    // Generate the repeating contnet
    for (const event of important_events) {
        
        // This requires fabricating a range
        for (let year = 0; year < YEARS_FROM_2022; year++) {
            let start_date = new Date(event.origin);
            let end_date = new Date(event.origin);

            start_date.setFullYear(start_date.getFullYear() + year, start_date.getMonth(), start_date.getDate());
            end_date.setFullYear(end_date.getFullYear() + year, end_date.getMonth(), end_date.getDate());
            end_date.setHours(23);
            end_date.setMinutes(59);
            end_date.setSeconds(59);

            lines.push(
                encode(SHADER_TYPE, event.shader, start_date, end_date, 255)
            );

            // Check if this event has a range
            if (event.range_start !== undefined) {
                // Generate the ranged entry
                let range_start = new Date(event.range_start);
                let range_end = new Date(event.range_end);
                range_start.setFullYear(range_start.getFullYear() + year, range_start.getMonth(), range_start.getDate());
                range_end.setFullYear(range_end.getFullYear() + year, range_end.getMonth(), range_end.getDate());
                range_end.setHours(23);
                range_end.setMinutes(59);
                range_end.setSeconds(59);
                
                lines.push(
                    encode(SHADER_TYPE, event.shader, range_start, range_end, 40)
                );
            }
        }
    }

    // Generate the one-off events
    for (const event of one_off_events) {
        let start_date = new Date(event.origin);
        let end_date = new Date(event.origin);
        end_date.setHours(23);
        end_date.setMinutes(59);
        end_date.setSeconds(59);

        lines.push(
            encode(SHADER_TYPE, event.shader, start_date, end_date, 255)
        );
    }
    
    return lines.join('\n');
};