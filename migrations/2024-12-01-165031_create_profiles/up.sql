/*
DO
$$
    BEGIN
        -- https://www.healthline.com/health/beauty-skin-care/skin-undertones#choosing-your-color-palette
        CREATE TYPE skin_undertone AS ENUM
            ('cool', 'warm', 'neutral', 'olive');

        -- https://www.schemecolor.com/real-skin-tones-color-palette.php
        CREATE TYPE skin_colour AS ENUM
            ('russet', 'peru', 'fawn', 'mellow_apricot', 'navajo_white');

        -- https://tsemrtd.js.org/enums/consts_enums.HairColor.html
        CREATE TYPE hair_colour AS ENUM (
            'bald', 'black', 'blond', 'brown',
            'gray', 'white', 'auburn', 'other');
    EXCEPTION
        WHEN duplicate_object THEN null;
    END
$$;
*/

CREATE TABLE "profiles"
(
    alias             VARCHAR(50) PRIMARY KEY,                                                    -- user-modifiable alias
    username          VARCHAR(50) NOT NULL REFERENCES users (username),                           -- (basically) immutable username
    profile_image_url VARCHAR(2048),                                                              -- profile image URL
    male              BOOLEAN,                                                                    -- whether individual is male
    height_mm         INTEGER CHECK (height_mm >= 0),                                             -- height in millimeters
    weight_g          INTEGER CHECK (weight_g >= 0),                                              -- weight in grams
    -- https://en.wikipedia.org/wiki/Bust/waist/hip_measurements
    bust_mm           INTEGER CHECK (bust_mm >= 0),                                               -- bust in millimeters
    waist_mm          INTEGER CHECK (waist_mm >= 0),                                              -- waist in millimeters
    hip_mm            INTEGER CHECK (hip_mm >= 0),                                                -- hip in millimeters
    -- the above measures can be inferred if given these sizes?:
    -- shirt size
    -- skirt size
    -- pant size
    -- (and vice versa)
    skin_undertone    VARCHAR(10) CHECK (skin_undertone in ('cool', 'warm', 'neutral', 'olive')), -- skin undertone ('cool', 'warm', 'neutral', or 'olive')
    skin_colour       VARCHAR(10) CHECK (skin_colour in ('russet', 'peru', 'fawn', 'mellow_apricot',
                                                         'navajo_white')),                        -- skin colour ('russet', 'peru', 'fawn', 'mellow_apricot', or 'navajo_white')
    hair_colour       VARCHAR(10) CHECK (hair_colour in ('bald', 'black', 'blond', 'brown',
                                                         'gray', 'white', 'auburn',
                                                         'other')),                               -- hair colour ('bald', 'black', 'blond', 'brown', 'gray', 'white', 'auburn', 'other')
    created_at        TIMESTAMP   NOT NULL DEFAULT current_timestamp                              -- when this record was first created
);
CREATE UNIQUE INDEX "profiles_alias_idx" ON profiles (alias, username);
