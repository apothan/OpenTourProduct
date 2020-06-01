use crate::schema::tour;
//use chrono::NaiveDateTime;
use diesel::MysqlConnection;
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "tour"]
pub struct Tour {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tours(pub Vec<Tour>);

#[derive(Serialize, Deserialize, Debug)]
pub struct TourForm {
    pub name: String,
}

impl Tours {
    pub fn list(connection: &MysqlConnection) -> Self {
        use crate::schema::tour::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = tour
            .limit(10)
            .load::<Tour>(connection)
            .expect("Error loading tours");

        Tours(result)
    }
    
    pub fn insert(
        tour_form: TourForm,
        connection: &MysqlConnection
    ) -> bool {
        
        let my_uuid = Uuid::new_v4();

        let new_tour = Tour {
            id : my_uuid.to_string(),
            name: tour_form.name
        };
        println!("recevied test call!");
        use crate::schema::tour::dsl::*;
        use diesel::insert_into;
        use diesel::RunQueryDsl;

        insert_into(tour).values(&new_tour).execute(connection);

        return true
    }
    
}