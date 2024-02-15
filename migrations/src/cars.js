const { Client } = require("pg");

(async () => {

    // node migration_file.js up/down [test]

    const command = process.argv[2].toLowerCase();

    const conn = new Client({
        connectionString: process.env.NODE_ENV === 'test' ? 
            `postgresql://root@root_db:26257/test_db` :
            process.env.DB_CONNECTION_STRING
    });
    await conn.connect();

    try{

        await conn.query("begin");
        switch(command){

            case "up":
            
                await conn.query(`
                    DO $$ BEGIN
                        create type car_t as enum('small', 'medium', 'large');
                    EXCEPTION
                        WHEN duplicate_object THEN null;
                    END $$;
                `)

                await conn.query(`
                    create table if not exists cars (
                        id varchar(32) primary key,
                        name varchar(255) not null,
                        plate_no varchar(7) not null,
                        tank_size int not null,
                        size car_t not null,
                        owner_id varchar(32) not null, -- owner's id
                        -- owner_data jsonb not null, -- (not needed for now) the entire user object, microservice friendly stuff
                        image_url varchar(255) default null
                    );
                `);
                await conn.query("commit");

            break;
    
            case "down":
                
                // await conn.query(`alter table users disable trigger all`);
                await conn.query(`drop table if exists cars cascade;`);
                await conn.query(`drop type if exists car_t;`);
                await conn.query("commit");

            break;
    
            default: throw Error('invalid command');
    
        }

    }catch(err){
        console.log(err);
        await conn.query("rollback");
    }finally{
        await conn.end();
    }


})();