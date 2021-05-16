-- these are GTFS tables taken from GTFS spec, while referencing MARTA's GTFS ZIP
-- google something like "gtfs reference" to get documentation on these
create table stops(
    stop_id INTEGER PRIMARY KEY,
    stop_code INTEGER,
    stop_name TEXT,
    stop_lat NUMERIC,
    stop_lon NUMERIC);
create table trips(
    trip_id INTEGER PRIMARY KEY,
    direction_id INTEGER,
    block_id INTEGER,
    shape_id INTEGER,
    route_id INTEGER,
    service_id INTEGER);
create table routes(
    route_id INTEGER PRIMARY KEY,
    route_short_name TEXT,
    route_long_name TEXT,
    route_desc TEXT,
    route_type INTEGER,
    route_url TEXT,
    route_text_color TEXT);
create table stop_times(
    trip_id INTEGER,
    arrival_time TEXT,
    departure_time TEXT,
    stop_id INTEGER,
    stop_sequence INTEGER,
    PRIMARY KEY(trip_id, stop_sequence)) WITHOUT ROWID;
create table calendar_dates(
    service_id INTEGER,
    date TEXT,
    exception_type INTEGER);
create table calendar(
    service_id INTEGER PRIMARY KEY,
    monday TINYINT,
    tuesday TINYINT,
    wednesday TINYINT,
    thursday TINYINT,
    friday TINYINT,
    saturday TINYINT,
    sunday TINYINT,
    start_date TEXT,
    end_date TEXT);