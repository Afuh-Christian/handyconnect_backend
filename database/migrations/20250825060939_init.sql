
-- CREATE TABLE IF NOT EXISTS public.appusers
-- (
--     id uuid NOT NULL DEFAULT gen_random_uuid(),
--     name character varying(50) COLLATE pg_catalog."default" NOT NULL,
--     email character varying(50) COLLATE pg_catalog."default" NOT NULL,
--     phone_number character varying(50) COLLATE pg_catalog."default",
--     password character varying(255) COLLATE pg_catalog."default" NOT NULL,
--     isHandyman boolean NOT NULL DEFAULT false, 
--     CONSTRAINT appusers_pkey PRIMARY KEY (id),
--     CONSTRAINT appusers_email_key UNIQUE (email)
-- );


--handymen

-- CREATE TABLE IF NOT EXISTS public.handymen
-- (
--     id uuid NOT NULL DEFAULT gen_random_uuid(),
--     name character varying(50) COLLATE pg_catalog."default" NOT NULL,
--     profession character varying(50) COLLATE pg_catalog."default",
--     email character varying(50) COLLATE pg_catalog."default" NOT NULL,
--     facebook character varying(50) COLLATE pg_catalog."default",
--     whatsapp character varying(50) COLLATE pg_catalog."default",
--     instagram character varying(50) COLLATE pg_catalog."default",
--     password character varying(255) COLLATE pg_catalog."default" NOT NULL,
--     years_of_experience character varying(50) COLLATE pg_catalog."default",
--     phone_number character varying(15) COLLATE pg_catalog."default",
--     CONSTRAINT handymen_pkey PRIMARY KEY (id),
--     CONSTRAINT handymen_email_key UNIQUE (email)
-- );



--location 






CREATE TABLE IF NOT EXISTS public.handymen
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_user_id UUID NOT NULL,
    location_id UUID NULL,
    profession_ids JSONB,
    ratings JSONB,
    contact_infos JSONB,
    payment_methods JSONB,
    CONSTRAINT fk_app_user FOREIGN KEY (app_user_id) REFERENCES public."AspNetUsers"("Id")
);

CREATE INDEX idx_handymen_profession_ids
ON public.handymen USING GIN (profession_ids);



--- location 


CREATE TABLE IF NOT EXISTS public.location
(
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    handyman_id uuid,
    address character varying(100) COLLATE pg_catalog."default",
    latitude numeric(9,6),
    longitude numeric(9,6),
    CONSTRAINT location_pkey PRIMARY KEY (id),
    CONSTRAINT location_handyman_id_key UNIQUE (handyman_id),
    CONSTRAINT location_handyman_id_fkey FOREIGN KEY (handyman_id)
        REFERENCES public."handymen" ("id") MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);











-- lookup_table

CREATE TABLE IF NOT EXISTS public.lookup_table (
    -- Primary Key: Uniquely identifies each lookup table.
    lookup_table_id INTEGER NOT NULL,
    language_lookup_data_id INTEGER NOT NULL,
    lookup_table_name VARCHAR(255) NOT NULL,

    CONSTRAINT lookup_table_pkey PRIMARY KEY (lookup_table_id , language_lookup_data_id)
);










-- lookup_data

-- lookup_table
--
--CREATE TABLE IF NOT EXISTS public.lookup_table (
--    -- Primary Key: Uniquely identifies each lookup table.
--    lookup_table_id INTEGER NOT NULL,
--    language_lookup_data_id INTEGER NOT NULL,
--    lookup_table_name VARCHAR(255) NOT NULL,
--
--    CONSTRAINT lookup_table_pkey PRIMARY KEY (lookup_table_id , language_lookup_data_id)
--);
--
--








-- The `lookup_data` table with the corrected foreign key constraint.
CREATE TABLE IF NOT EXISTS public.lookup_data (
    lookup_data_id INTEGER NOT NULL,
    lookup_table_id INTEGER NOT NULL,
    language_lookup_data_id INTEGER NOT NULL,
    lookup_data_name VARCHAR(255) NOT NULL,
    description TEXT , 
    
    CONSTRAINT lookup_data_pkey PRIMARY KEY (lookup_data_id, language_lookup_data_id),
    
    -- Corrected foreign key: It must reference the full composite primary key
    -- of the `lookup_table` table, which is (lookup_table_id, language_lookup_data_id).
    CONSTRAINT fk_lookup_table_id
        FOREIGN KEY (lookup_table_id, language_lookup_data_id)
        REFERENCES public.lookup_table(lookup_table_id, language_lookup_data_id)
);









CREATE TABLE IF NOT EXISTS public.ratings (
    -- A composite primary key is a combination of two or more columns
    -- that can be used to uniquely identify each row in a table.
    -- In this case, the combination of id, handyman_id, and profession_id
    -- forms the primary key, as you requested.
    handyman_id UUID NOT NULL,
    profession_id UUID NOT NULL,

    app_user_id UUID NOT NULL,
    comment TEXT,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    score INTEGER NOT NULL,

    -- Define the composite primary key
    CONSTRAINT rating_pkey PRIMARY KEY (app_user_id , handyman_id, profession_id),

    -- Add foreign key constraints to link to other tables for data integrity
    CONSTRAINT fk_user
        FOREIGN KEY (app_user_id) REFERENCES public."AspNetUsers" ("Id"),
    CONSTRAINT fk_handyman
        FOREIGN KEY (handyman_id) REFERENCES public.handymen(id)
--        ,
--    CONSTRAINT fk_profession
--        FOREIGN KEY (profession_id) REFERENCES public.lookup_data(lookup_data_id)
);














--- transactions .. 

-- This SQL script creates the `transactions` table.
CREATE TABLE IF NOT EXISTS public.transactions (
    -- Primary Key: A unique identifier for each transaction.
    transaction_id UUID PRIMARY KEY,

    -- A unique, human-readable identifier for the transaction.
    transaction_number VARCHAR(255) UNIQUE NOT NULL,

    -- Foreign Key: The user or entity who sent the transaction.
    sender_id UUID,
    
    -- Foreign Key: The user or entity who received the transaction.
    recipient_id UUID,

    -- The transaction amount, using a floating-point type.
    amount FLOAT NOT NULL,

    -- The fee charged by the platform.
    platform_fee FLOAT,

    -- The amount the recipient actually receives after fees.
    recipient_earnings FLOAT,

    -- Foreign Key: The currency used in the transaction.
    currency_id INTEGER NOT NULL,

    -- Foreign Key: The current status of the transaction (e.g., pending, completed).
    transaction_status_id INTEGER NOT NULL,

    -- Foreign Key: The payment method used (e.g., credit card, mobile money).
    payment_method_id INTEGER NOT NULL,

    -- Foreign Key: The third-party payment provider used.
    provider_id INTEGER,

    -- A reference string for the transaction, often from the payment provider.
    reference VARCHAR(255) NOT NULL,

    -- Optional: The account number of the payer.
    payer_account VARCHAR(255),

    -- Optional: The account number of the recipient.
    recipient_account VARCHAR(255),

    -- Timestamps for when the transaction was created and last updated.
    -- `TIMESTAMP WITH TIME ZONE` is used to correctly handle `DateTime<Utc>`.
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    -- Add foreign key constraints to link to other tables.
    CONSTRAINT fk_sender
        FOREIGN KEY (sender_id) REFERENCES public."AspNetUsers"("Id"),
    CONSTRAINT fk_recipient
        FOREIGN KEY (recipient_id) REFERENCES public."AspNetUsers"("Id")
--        ,
--    CONSTRAINT fk_currency
--        FOREIGN KEY (currency_id) REFERENCES public.lookup_data(lookup_data_id),
--    CONSTRAINT fk_transaction_status
--        FOREIGN KEY (transaction_status_id) REFERENCES public.lookup_data(lookup_data_id),
--    CONSTRAINT fk_payment_method
--        FOREIGN KEY (payment_method_id) REFERENCES public.lookup_data(lookup_data_id),
--    CONSTRAINT fk_provider
--        FOREIGN KEY (provider_id) REFERENCES public.lookup_data(lookup_data_id)
);




--messages

-- CREATE TABLE IF NOT EXISTS public.messages
-- (
--     id uuid NOT NULL DEFAULT gen_random_uuid(),
--     sender_id uuid,
--     receiver_id uuid,
--     content character varying(255) COLLATE pg_catalog."default" NOT NULL,
--     "timestamp" character varying(50) COLLATE pg_catalog."default" NOT NULL,
--     CONSTRAINT messages_pkey PRIMARY KEY (id),
--     CONSTRAINT messages_receiver_id_fkey FOREIGN KEY (receiver_id)
--         REFERENCES public.appusers (id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE CASCADE,
--     CONSTRAINT messages_sender_id_fkey FOREIGN KEY (sender_id)
--         REFERENCES public.appusers (id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE CASCADE
-- );



--permissions

-- CREATE TABLE IF NOT EXISTS public.permissions
-- (
--     id uuid NOT NULL DEFAULT gen_random_uuid(),
--     name character varying(50) COLLATE pg_catalog."default" NOT NULL,
--     description text COLLATE pg_catalog."default",
--     CONSTRAINT permissions_pkey PRIMARY KEY (id),
--     CONSTRAINT permissions_name_key UNIQUE (name)
-- );





--professions

-- CREATE TABLE IF NOT EXISTS public.professions
-- (
--     id uuid NOT NULL DEFAULT gen_random_uuid(),
--     name character varying(50) COLLATE pg_catalog."default" NOT NULL,
--     description character varying(255) COLLATE pg_catalog."default",
--     handyman_id uuid,
--     CONSTRAINT professions_pkey PRIMARY KEY (id),
--     CONSTRAINT professions_handyman_id_key UNIQUE (handyman_id),
--     CONSTRAINT professions_name_key UNIQUE (name),
--     CONSTRAINT professions_handyman_id_fkey FOREIGN KEY (handyman_id)
--         REFERENCES public.handymen (id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE CASCADE
-- );



--ratings

-- CREATE TABLE IF NOT EXISTS public.ratings
-- (
--     id uuid NOT NULL DEFAULT gen_random_uuid(),
--     user_id uuid,
--     comment character varying(255) COLLATE pg_catalog."default",
--     score integer,
--     CONSTRAINT ratings_pkey PRIMARY KEY (id),
--     CONSTRAINT ratings_user_id_fkey FOREIGN KEY (user_id)
--         REFERENCES public.appusers (id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE CASCADE,
--     CONSTRAINT ratings_score_check CHECK (score >= 1 AND score <= 5)
-- );


--roles

-- CREATE TABLE IF NOT EXISTS public.roles
-- (
--     id uuid NOT NULL DEFAULT gen_random_uuid(),
--     name character varying(50) COLLATE pg_catalog."default" NOT NULL,
--     description text COLLATE pg_catalog."default",
--     CONSTRAINT roles_pkey PRIMARY KEY (id),
--     CONSTRAINT roles_name_key UNIQUE (name)
-- );



--rolepermissions

-- CREATE TABLE IF NOT EXISTS public.rolepermissions
-- (
--     id uuid NOT NULL DEFAULT gen_random_uuid(),
--     role_id uuid,
--     permission_id uuid,
--     CONSTRAINT rolepermissions_pkey PRIMARY KEY (id),
--     CONSTRAINT rolepermissions_role_id_permission_id_key UNIQUE (role_id, permission_id),
--     CONSTRAINT rolepermissions_permission_id_fkey FOREIGN KEY (permission_id)
--         REFERENCES public.permissions (id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE CASCADE,
--     CONSTRAINT rolepermissions_role_id_fkey FOREIGN KEY (role_id)
--         REFERENCES public.roles (id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE CASCADE
-- );







--userroles

-- CREATE TABLE IF NOT EXISTS public.userroles
-- (
--     id uuid NOT NULL DEFAULT gen_random_uuid(),
--     user_id uuid,
--     role_id uuid,
--     CONSTRAINT userroles_pkey PRIMARY KEY (id),
--     CONSTRAINT userroles_user_id_role_id_key UNIQUE (user_id, role_id),
--     CONSTRAINT userroles_role_id_fkey FOREIGN KEY (role_id)
--         REFERENCES public.roles (id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE CASCADE,
--     CONSTRAINT userroles_user_id_fkey FOREIGN KEY (user_id)
--         REFERENCES public.appusers (id) MATCH SIMPLE
--         ON UPDATE NO ACTION
--         ON DELETE CASCADE
-- );



--users
/*
CREATE TABLE IF NOT EXISTS public.users
(
    id integer NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    username character varying(100) COLLATE pg_catalog."default" NOT NULL,
    email character varying(100) COLLATE pg_catalog."default" NOT NULL,
    password character varying(255) COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT users_pkey PRIMARY KEY (id),
    CONSTRAINT users_email_key UNIQUE (email)
);

*/


