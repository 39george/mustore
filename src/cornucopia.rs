// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { pub mod public { #[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum Musickey { a_minor,a_major,b_flat_minor,b_flat_major,b_minor,b_major,c_minor,c_major,c_sharp_minor,c_sharp_major,d_minor,d_major,e_flat_minor,e_flat_major,e_minor,e_major,f_minor,f_major,f_sharp_minor,f_sharp_major,g_minor,g_major,a_flat_minor,a_flat_major,}impl < 'a > postgres_types :: ToSql for Musickey
{
    fn
    to_sql(& self, ty : & postgres_types :: Type, buf : & mut postgres_types
    :: private :: BytesMut,) -> Result < postgres_types :: IsNull, Box < dyn
    std :: error :: Error + Sync + Send >, >
    {
        let s = match * self { Musickey :: a_minor => "a_minor",Musickey :: a_major => "a_major",Musickey :: b_flat_minor => "b_flat_minor",Musickey :: b_flat_major => "b_flat_major",Musickey :: b_minor => "b_minor",Musickey :: b_major => "b_major",Musickey :: c_minor => "c_minor",Musickey :: c_major => "c_major",Musickey :: c_sharp_minor => "c_sharp_minor",Musickey :: c_sharp_major => "c_sharp_major",Musickey :: d_minor => "d_minor",Musickey :: d_major => "d_major",Musickey :: e_flat_minor => "e_flat_minor",Musickey :: e_flat_major => "e_flat_major",Musickey :: e_minor => "e_minor",Musickey :: e_major => "e_major",Musickey :: f_minor => "f_minor",Musickey :: f_major => "f_major",Musickey :: f_sharp_minor => "f_sharp_minor",Musickey :: f_sharp_major => "f_sharp_major",Musickey :: g_minor => "g_minor",Musickey :: g_major => "g_major",Musickey :: a_flat_minor => "a_flat_minor",Musickey :: a_flat_major => "a_flat_major",}
        ; buf.extend_from_slice(s.as_bytes()) ; std :: result :: Result ::
        Ok(postgres_types :: IsNull :: No)
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != "musickey" { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Enum(ref variants) =>
            {
                if variants.len() != 24 { return false ; }
                variants.iter().all(| v | match & * * v
                { "a_minor" => true,"a_major" => true,"b_flat_minor" => true,"b_flat_major" => true,"b_minor" => true,"b_major" => true,"c_minor" => true,"c_major" => true,"c_sharp_minor" => true,"c_sharp_major" => true,"d_minor" => true,"d_major" => true,"e_flat_minor" => true,"e_flat_major" => true,"e_minor" => true,"e_major" => true,"f_minor" => true,"f_major" => true,"f_sharp_minor" => true,"f_sharp_major" => true,"g_minor" => true,"g_major" => true,"a_flat_minor" => true,"a_flat_major" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(& self, ty : & postgres_types :: Type, out : & mut
    postgres_types :: private :: BytesMut,) -> Result < postgres_types ::
    IsNull, Box < dyn std :: error :: Error + Sync + Send >>
    { postgres_types :: __to_sql_checked(self, ty, out) }
} impl < 'a > postgres_types :: FromSql < 'a > for Musickey
{
    fn from_sql(ty : & postgres_types :: Type, buf : & 'a [u8],) -> Result <
    Musickey, Box < dyn std :: error :: Error + Sync + Send >, >
    {
        match std :: str :: from_utf8(buf) ?
        {
            "a_minor" => Ok(Musickey :: a_minor),"a_major" => Ok(Musickey :: a_major),"b_flat_minor" => Ok(Musickey :: b_flat_minor),"b_flat_major" => Ok(Musickey :: b_flat_major),"b_minor" => Ok(Musickey :: b_minor),"b_major" => Ok(Musickey :: b_major),"c_minor" => Ok(Musickey :: c_minor),"c_major" => Ok(Musickey :: c_major),"c_sharp_minor" => Ok(Musickey :: c_sharp_minor),"c_sharp_major" => Ok(Musickey :: c_sharp_major),"d_minor" => Ok(Musickey :: d_minor),"d_major" => Ok(Musickey :: d_major),"e_flat_minor" => Ok(Musickey :: e_flat_minor),"e_flat_major" => Ok(Musickey :: e_flat_major),"e_minor" => Ok(Musickey :: e_minor),"e_major" => Ok(Musickey :: e_major),"f_minor" => Ok(Musickey :: f_minor),"f_major" => Ok(Musickey :: f_major),"f_sharp_minor" => Ok(Musickey :: f_sharp_minor),"f_sharp_major" => Ok(Musickey :: f_sharp_major),"g_minor" => Ok(Musickey :: g_minor),"g_major" => Ok(Musickey :: g_major),"a_flat_minor" => Ok(Musickey :: a_flat_minor),"a_flat_major" => Ok(Musickey :: a_flat_major),s => Result ::
            Err(Into :: into(format! ("invalid variant `{}`", s))),
        }
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != "musickey" { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Enum(ref variants) =>
            {
                if variants.len() != 24 { return false ; }
                variants.iter().all(| v | match & * * v
                { "a_minor" => true,"a_major" => true,"b_flat_minor" => true,"b_flat_major" => true,"b_minor" => true,"b_major" => true,"c_minor" => true,"c_major" => true,"c_sharp_minor" => true,"c_sharp_major" => true,"d_minor" => true,"d_major" => true,"e_flat_minor" => true,"e_flat_major" => true,"e_minor" => true,"e_major" => true,"f_minor" => true,"f_major" => true,"f_sharp_minor" => true,"f_sharp_major" => true,"g_minor" => true,"g_major" => true,"a_flat_minor" => true,"a_flat_major" => true,_ => false, })
            } _ => false,
        }
    }
} }}#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod creator_access
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertProductAndGetProductIdParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> { pub owher_id : i32,pub name : T1,pub description : Option<T2>,pub price : rust_decimal::Decimal,}#[derive( Debug)] pub struct InsertProductCoverObjectKeyParams < T1 : cornucopia_async::StringSql,> { pub key : T1,pub product_id : i32,}#[derive( Debug)] pub struct InsertProductMoodByNameParams < T1 : cornucopia_async::StringSql,> { pub product_id : i32,pub mood_name : T1,}#[derive( Debug)] pub struct InsertSongAndGetSongIdParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,> { pub product_id : i32,pub primary_genre : T1,pub secondary_genre : Option<T2>,pub sex : T3,pub tempo : i16,pub key : super::super::types::public::Musickey,pub duration : i16,pub lyric : T4,}#[derive( Debug)] pub struct InsertBeatAndGetBeatIdParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> { pub product_id : i32,pub primary_genre : T1,pub secondary_genre : Option<T2>,pub tempo : i16,pub key : super::super::types::public::Musickey,pub duration : i16,}#[derive( Debug)] pub struct InsertMusicProductMasterObjectKeyParams < T1 : cornucopia_async::StringSql,> { pub key : T1,pub song_id : Option<i32>,pub beat_id : Option<i32>,}#[derive( Debug)] pub struct InsertMusicProductMasterTaggedObjectKeyParams < T1 : cornucopia_async::StringSql,> { pub key : T1,pub song_id : Option<i32>,pub beat_id : Option<i32>,}#[derive( Debug)] pub struct InsertMusicProductMultitrackObjectKeyParams < T1 : cornucopia_async::StringSql,> { pub key : T1,pub song_id : Option<i32>,pub beat_id : Option<i32>,}#[derive( Debug)] pub struct CreateOfferParams < T1 : cornucopia_async::StringSql,> { pub conversations_id : i32,pub services_id : i32,pub text : T1,pub price : rust_decimal::Decimal,pub delivery_date : time::OffsetDateTime,pub free_refisions : i32,pub revision_price : rust_decimal::Decimal,}#[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)] pub struct GetCreatorMarksAvg
{ pub avg : rust_decimal::Decimal,pub count : i64,}pub struct GetCreatorMarksAvgQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetCreatorMarksAvg,
    mapper : fn(GetCreatorMarksAvg) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetCreatorMarksAvgQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetCreatorMarksAvg) -> R) -> GetCreatorMarksAvgQuery
    < 'a, C, R, N >
    {
        GetCreatorMarksAvgQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetCreatorInboxResponseRateAndTime
{ pub response_rate_percentage : f64,pub average_response_time : String,}pub struct GetCreatorInboxResponseRateAndTimeBorrowed < 'a >
{ pub response_rate_percentage : f64,pub average_response_time : &'a str,} impl < 'a > From < GetCreatorInboxResponseRateAndTimeBorrowed <
'a >> for GetCreatorInboxResponseRateAndTime
{
    fn
    from(GetCreatorInboxResponseRateAndTimeBorrowed { response_rate_percentage,average_response_time,} : GetCreatorInboxResponseRateAndTimeBorrowed < 'a >)
    -> Self { Self { response_rate_percentage,average_response_time: average_response_time.into(),} }
}pub struct GetCreatorInboxResponseRateAndTimeQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetCreatorInboxResponseRateAndTimeBorrowed,
    mapper : fn(GetCreatorInboxResponseRateAndTimeBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetCreatorInboxResponseRateAndTimeQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetCreatorInboxResponseRateAndTimeBorrowed) -> R) -> GetCreatorInboxResponseRateAndTimeQuery
    < 'a, C, R, N >
    {
        GetCreatorInboxResponseRateAndTimeQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub struct I32Query < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> i32,
    mapper : fn(i32) -> T,
} impl < 'a, C, T : 'a, const N : usize > I32Query < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(i32) -> R) -> I32Query
    < 'a, C, R, N >
    {
        I32Query
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn get_creator_marks_avg() -> GetCreatorMarksAvgStmt
{ GetCreatorMarksAvgStmt(cornucopia_async :: private :: Stmt :: new("SELECT AVG(mark), COUNT(mark)
FROM service_reviews
JOIN service_orders
ON service_reviews.service_orders_id = service_orders.id
JOIN services
ON service_orders.services_id = services.id
WHERE services.creator_id = $1")) } pub
struct GetCreatorMarksAvgStmt(cornucopia_async :: private :: Stmt) ; impl
GetCreatorMarksAvgStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
creator_id : & 'a i32,) -> GetCreatorMarksAvgQuery < 'a, C,
GetCreatorMarksAvg, 1 >
{
    GetCreatorMarksAvgQuery
    {
        client, params : [creator_id,], stmt : & mut self.0, extractor :
        | row | { GetCreatorMarksAvg { avg : row.get(0),count : row.get(1),} }, mapper : | it | { <GetCreatorMarksAvg>::from(it) },
    }
} }pub fn get_creator_inbox_response_rate_and_time() -> GetCreatorInboxResponseRateAndTimeStmt
{ GetCreatorInboxResponseRateAndTimeStmt(cornucopia_async :: private :: Stmt :: new("WITH FirstResponseTime AS (
    SELECT
        conversations.id AS conversation_id,
        MIN(messages.created_at) AS first_response_time
    FROM conversations
    JOIN participants ON conversations.id = participants.conversations_id
    JOIN messages ON conversations.id = messages.conversations_id
    WHERE participants.users_id = $1
        AND messages.users_id = $1
        AND messages.created_at > conversations.created_at
    GROUP BY conversations.id
),
ConversationResponses AS (
    SELECT
        conversations.id,
        (CASE
            WHEN frt.first_response_time IS NOT NULL AND 
                 frt.first_response_time - conversations.created_at < INTERVAL '1 day' 
            THEN 1
            ELSE 0 
         END) AS is_responded,
        frt.first_response_time - conversations.created_at AS response_time
    FROM conversations
    LEFT JOIN FirstResponseTime frt ON conversations.id = frt.conversation_id
    WHERE EXISTS (
        SELECT 1
        FROM messages
        WHERE messages.conversations_id = conversations.id
          AND messages.users_id <> $1
        LIMIT 1
    )
    AND conversations.created_at > NOW() - INTERVAL '1 month'
)
SELECT
    COALESCE(
        -- COUNT() will NOT count NULLS
       (COUNT(CASE WHEN is_responded = 1 THEN 1 END)::float / COUNT(*)::float) * 100,
       0
    ) AS response_rate_percentage,
    AVG(response_time)::TEXT AS average_response_time
FROM ConversationResponses")) } pub
struct GetCreatorInboxResponseRateAndTimeStmt(cornucopia_async :: private :: Stmt) ; impl
GetCreatorInboxResponseRateAndTimeStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
user_id : & 'a i32,) -> GetCreatorInboxResponseRateAndTimeQuery < 'a, C,
GetCreatorInboxResponseRateAndTime, 1 >
{
    GetCreatorInboxResponseRateAndTimeQuery
    {
        client, params : [user_id,], stmt : & mut self.0, extractor :
        | row | { GetCreatorInboxResponseRateAndTimeBorrowed { response_rate_percentage : row.get(0),average_response_time : row.get(1),} }, mapper : | it | { <GetCreatorInboxResponseRateAndTime>::from(it) },
    }
} }pub fn get_profile_completion_value() -> GetProfileCompletionValueStmt
{ GetProfileCompletionValueStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
    CASE
        WHEN bio IS NULL THEN 80
        ELSE 100
    END AS profile_completion_value
FROM users
WHERE users.id = $1")) } pub
struct GetProfileCompletionValueStmt(cornucopia_async :: private :: Stmt) ; impl
GetProfileCompletionValueStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
user_id : & 'a i32,) -> I32Query < 'a, C,
i32, 1 >
{
    I32Query
    {
        client, params : [user_id,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }pub fn insert_product_and_get_product_id() -> InsertProductAndGetProductIdStmt
{ InsertProductAndGetProductIdStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO products(owner_id, name, description, price)
VALUES ($1, $2, $3, $4) returning id")) } pub
struct InsertProductAndGetProductIdStmt(cornucopia_async :: private :: Stmt) ; impl
InsertProductAndGetProductIdStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
owher_id : & 'a i32,name : & 'a T1,description : & 'a Option<T2>,price : & 'a rust_decimal::Decimal,) -> I32Query < 'a, C,
i32, 4 >
{
    I32Query
    {
        client, params : [owher_id,name,description,price,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, InsertProductAndGetProductIdParams < T1,T2,>, I32Query < 'a,
C, i32, 4 >, C > for InsertProductAndGetProductIdStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertProductAndGetProductIdParams < T1,T2,>) -> I32Query < 'a, C,
    i32, 4 >
    { self.bind(client, & params.owher_id,& params.name,& params.description,& params.price,) }
}pub fn insert_product_cover_object_key() -> InsertProductCoverObjectKeyStmt
{ InsertProductCoverObjectKeyStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO objects(key, object_type, cover_products_id)
VALUES ($1, 'image', $2)")) } pub
struct InsertProductCoverObjectKeyStmt(cornucopia_async :: private :: Stmt) ; impl
InsertProductCoverObjectKeyStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
key : & 'a T1,product_id : & 'a i32,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [key,product_id,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, InsertProductCoverObjectKeyParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for InsertProductCoverObjectKeyStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertProductCoverObjectKeyParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.key,& params.product_id,) ) }
}pub fn insert_product_mood_by_name() -> InsertProductMoodByNameStmt
{ InsertProductMoodByNameStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO products_moods (products_id, moods_id)
VALUES ($1, (
    SELECT id FROM moods WHERE name = $2
))")) } pub
struct InsertProductMoodByNameStmt(cornucopia_async :: private :: Stmt) ; impl
InsertProductMoodByNameStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
product_id : & 'a i32,mood_name : & 'a T1,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [product_id,mood_name,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, InsertProductMoodByNameParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for InsertProductMoodByNameStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertProductMoodByNameParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.product_id,& params.mood_name,) ) }
}pub fn insert_song_and_get_song_id() -> InsertSongAndGetSongIdStmt
{ InsertSongAndGetSongIdStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO songs (
    products_id,
    primary_genre,
    secondary_genre,
    sex,
    tempo,
    key,
    duration,
    lyric
)
VALUES (
    $1,
    (SELECT id FROM genres WHERE name = $2),
    (
        CASE
            WHEN $3::VARCHAR(50) IS NOT NULL THEN
                (SELECT id FROM genres WHERE name = $3)
        END
    ),
    $4,
    $5,
    $6,
    $7,
    $8
)
RETURNING id")) } pub
struct InsertSongAndGetSongIdStmt(cornucopia_async :: private :: Stmt) ; impl
InsertSongAndGetSongIdStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
product_id : & 'a i32,primary_genre : & 'a T1,secondary_genre : & 'a Option<T2>,sex : & 'a T3,tempo : & 'a i16,key : & 'a super::super::types::public::Musickey,duration : & 'a i16,lyric : & 'a T4,) -> I32Query < 'a, C,
i32, 8 >
{
    I32Query
    {
        client, params : [product_id,primary_genre,secondary_genre,sex,tempo,key,duration,lyric,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, InsertSongAndGetSongIdParams < T1,T2,T3,T4,>, I32Query < 'a,
C, i32, 8 >, C > for InsertSongAndGetSongIdStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertSongAndGetSongIdParams < T1,T2,T3,T4,>) -> I32Query < 'a, C,
    i32, 8 >
    { self.bind(client, & params.product_id,& params.primary_genre,& params.secondary_genre,& params.sex,& params.tempo,& params.key,& params.duration,& params.lyric,) }
}pub fn insert_beat_and_get_beat_id() -> InsertBeatAndGetBeatIdStmt
{ InsertBeatAndGetBeatIdStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO beats (
    products_id,
    primary_genre,
    secondary_genre,
    tempo,
    key,
    duration
)
VALUES (
    $1,
    (SELECT id FROM genres WHERE name = $2),
    (
        CASE
            WHEN $3::VARCHAR(50) IS NOT NULL THEN
                (SELECT id FROM genres WHERE name = $3)
        END
    ),
    $4,
    $5,
    $6
)
RETURNING id")) } pub
struct InsertBeatAndGetBeatIdStmt(cornucopia_async :: private :: Stmt) ; impl
InsertBeatAndGetBeatIdStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
product_id : & 'a i32,primary_genre : & 'a T1,secondary_genre : & 'a Option<T2>,tempo : & 'a i16,key : & 'a super::super::types::public::Musickey,duration : & 'a i16,) -> I32Query < 'a, C,
i32, 6 >
{
    I32Query
    {
        client, params : [product_id,primary_genre,secondary_genre,tempo,key,duration,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, InsertBeatAndGetBeatIdParams < T1,T2,>, I32Query < 'a,
C, i32, 6 >, C > for InsertBeatAndGetBeatIdStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertBeatAndGetBeatIdParams < T1,T2,>) -> I32Query < 'a, C,
    i32, 6 >
    { self.bind(client, & params.product_id,& params.primary_genre,& params.secondary_genre,& params.tempo,& params.key,& params.duration,) }
}pub fn insert_music_product_master_object_key() -> InsertMusicProductMasterObjectKeyStmt
{ InsertMusicProductMasterObjectKeyStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO objects(key, object_type, master_songs_id, master_beats_id)
VALUES ($1, 'audio', $2, $3)")) } pub
struct InsertMusicProductMasterObjectKeyStmt(cornucopia_async :: private :: Stmt) ; impl
InsertMusicProductMasterObjectKeyStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
key : & 'a T1,song_id : & 'a Option<i32>,beat_id : & 'a Option<i32>,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [key,song_id,beat_id,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, InsertMusicProductMasterObjectKeyParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for InsertMusicProductMasterObjectKeyStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertMusicProductMasterObjectKeyParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.key,& params.song_id,& params.beat_id,) ) }
}pub fn insert_music_product_master_tagged_object_key() -> InsertMusicProductMasterTaggedObjectKeyStmt
{ InsertMusicProductMasterTaggedObjectKeyStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO objects(key, object_type, tagged_master_songs_id, tagged_master_beats_id)
VALUES ($1, 'audio', $2, $3)")) } pub
struct InsertMusicProductMasterTaggedObjectKeyStmt(cornucopia_async :: private :: Stmt) ; impl
InsertMusicProductMasterTaggedObjectKeyStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
key : & 'a T1,song_id : & 'a Option<i32>,beat_id : & 'a Option<i32>,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [key,song_id,beat_id,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, InsertMusicProductMasterTaggedObjectKeyParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for InsertMusicProductMasterTaggedObjectKeyStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertMusicProductMasterTaggedObjectKeyParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.key,& params.song_id,& params.beat_id,) ) }
}pub fn insert_music_product_multitrack_object_key() -> InsertMusicProductMultitrackObjectKeyStmt
{ InsertMusicProductMultitrackObjectKeyStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO objects(key, object_type, multitrack_songs_id, multitrack_beats_id)
VALUES ($1, 'multitrack', $2, $3)")) } pub
struct InsertMusicProductMultitrackObjectKeyStmt(cornucopia_async :: private :: Stmt) ; impl
InsertMusicProductMultitrackObjectKeyStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
key : & 'a T1,song_id : & 'a Option<i32>,beat_id : & 'a Option<i32>,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [key,song_id,beat_id,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, InsertMusicProductMultitrackObjectKeyParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for InsertMusicProductMultitrackObjectKeyStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertMusicProductMultitrackObjectKeyParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.key,& params.song_id,& params.beat_id,) ) }
}pub fn create_offer() -> CreateOfferStmt
{ CreateOfferStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO offers(conversations_id, services_id, text, price, delivery_date, free_revisions, revision_price)
VALUES ($1, $2, $3, $4, $5, $6, $7)")) } pub
struct CreateOfferStmt(cornucopia_async :: private :: Stmt) ; impl
CreateOfferStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
conversations_id : & 'a i32,services_id : & 'a i32,text : & 'a T1,price : & 'a rust_decimal::Decimal,delivery_date : & 'a time::OffsetDateTime,free_refisions : & 'a i32,revision_price : & 'a rust_decimal::Decimal,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [conversations_id,services_id,text,price,delivery_date,free_refisions,revision_price,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, CreateOfferParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for CreateOfferStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    CreateOfferParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.conversations_id,& params.services_id,& params.text,& params.price,& params.delivery_date,& params.free_refisions,& params.revision_price,) ) }
}}pub mod internal
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;pub fn refresh_available_songs() -> RefreshAvailableSongsStmt
{ RefreshAvailableSongsStmt(cornucopia_async :: private :: Stmt :: new("REFRESH MATERIALIZED VIEW available_songs")) } pub
struct RefreshAvailableSongsStmt(cornucopia_async :: private :: Stmt) ; impl
RefreshAvailableSongsStmt { pub async fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & []) .await
} }}pub mod open_access
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct GetSongsParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::ArraySql<Item = i16>,T3 : cornucopia_async::ArraySql<Item = super::super::types::public::Musickey>,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::ArraySql<Item = T4>,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::ArraySql<Item = T6>,T8 : cornucopia_async::StringSql,> { pub user_id : Option<i32>,pub sex : Option<T1>,pub tempo : Option<T2>,pub key : Option<T3>,pub genre : Option<T5>,pub mood : Option<T7>,pub sort_by : T8,pub offset : i64,pub amount : i64,}#[derive(Clone,Copy, Debug)] pub struct GetNewSongsParams < > { pub user_id : Option<i32>,pub amount : i64,}#[derive(Clone,Copy, Debug)] pub struct GetRecommendedSongsParams < > { pub user_id : Option<i32>,pub amount : i64,}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetStats
{ pub table_name : String,pub count : i64,}pub struct GetStatsBorrowed < 'a >
{ pub table_name : &'a str,pub count : i64,} impl < 'a > From < GetStatsBorrowed <
'a >> for GetStats
{
    fn
    from(GetStatsBorrowed { table_name,count,} : GetStatsBorrowed < 'a >)
    -> Self { Self { table_name: table_name.into(),count,} }
}pub struct GetStatsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetStatsBorrowed,
    mapper : fn(GetStatsBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetStatsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetStatsBorrowed) -> R) -> GetStatsQuery
    < 'a, C, R, N >
    {
        GetStatsQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub struct StringQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> & str,
    mapper : fn(& str) -> T,
} impl < 'a, C, T : 'a, const N : usize > StringQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(& str) -> R) -> StringQuery
    < 'a, C, R, N >
    {
        StringQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetSongs
{ pub song_id : i32,pub created_at : time::OffsetDateTime,pub cover_url : String,pub name : String,pub author : String,pub likes : i64,pub listenings : i64,pub relevance_score : rust_decimal::Decimal,pub price : rust_decimal::Decimal,pub is_user_liked : Option<bool>,}pub struct GetSongsBorrowed < 'a >
{ pub song_id : i32,pub created_at : time::OffsetDateTime,pub cover_url : &'a str,pub name : &'a str,pub author : &'a str,pub likes : i64,pub listenings : i64,pub relevance_score : rust_decimal::Decimal,pub price : rust_decimal::Decimal,pub is_user_liked : Option<bool>,} impl < 'a > From < GetSongsBorrowed <
'a >> for GetSongs
{
    fn
    from(GetSongsBorrowed { song_id,created_at,cover_url,name,author,likes,listenings,relevance_score,price,is_user_liked,} : GetSongsBorrowed < 'a >)
    -> Self { Self { song_id,created_at,cover_url: cover_url.into(),name: name.into(),author: author.into(),likes,listenings,relevance_score,price,is_user_liked,} }
}pub struct GetSongsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetSongsBorrowed,
    mapper : fn(GetSongsBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetSongsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetSongsBorrowed) -> R) -> GetSongsQuery
    < 'a, C, R, N >
    {
        GetSongsQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetNewSongs
{ pub song_id : i32,pub created_at : time::OffsetDateTime,pub cover_url : String,pub name : String,pub author : String,pub likes : i64,pub price : rust_decimal::Decimal,pub is_user_liked : Option<bool>,}pub struct GetNewSongsBorrowed < 'a >
{ pub song_id : i32,pub created_at : time::OffsetDateTime,pub cover_url : &'a str,pub name : &'a str,pub author : &'a str,pub likes : i64,pub price : rust_decimal::Decimal,pub is_user_liked : Option<bool>,} impl < 'a > From < GetNewSongsBorrowed <
'a >> for GetNewSongs
{
    fn
    from(GetNewSongsBorrowed { song_id,created_at,cover_url,name,author,likes,price,is_user_liked,} : GetNewSongsBorrowed < 'a >)
    -> Self { Self { song_id,created_at,cover_url: cover_url.into(),name: name.into(),author: author.into(),likes,price,is_user_liked,} }
}pub struct GetNewSongsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetNewSongsBorrowed,
    mapper : fn(GetNewSongsBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetNewSongsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetNewSongsBorrowed) -> R) -> GetNewSongsQuery
    < 'a, C, R, N >
    {
        GetNewSongsQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetRecommendedSongs
{ pub song_id : i32,pub created_at : time::OffsetDateTime,pub cover_url : String,pub name : String,pub author : String,pub likes : i64,pub price : rust_decimal::Decimal,pub is_user_liked : Option<bool>,}pub struct GetRecommendedSongsBorrowed < 'a >
{ pub song_id : i32,pub created_at : time::OffsetDateTime,pub cover_url : &'a str,pub name : &'a str,pub author : &'a str,pub likes : i64,pub price : rust_decimal::Decimal,pub is_user_liked : Option<bool>,} impl < 'a > From < GetRecommendedSongsBorrowed <
'a >> for GetRecommendedSongs
{
    fn
    from(GetRecommendedSongsBorrowed { song_id,created_at,cover_url,name,author,likes,price,is_user_liked,} : GetRecommendedSongsBorrowed < 'a >)
    -> Self { Self { song_id,created_at,cover_url: cover_url.into(),name: name.into(),author: author.into(),likes,price,is_user_liked,} }
}pub struct GetRecommendedSongsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetRecommendedSongsBorrowed,
    mapper : fn(GetRecommendedSongsBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetRecommendedSongsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetRecommendedSongsBorrowed) -> R) -> GetRecommendedSongsQuery
    < 'a, C, R, N >
    {
        GetRecommendedSongsQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn get_stats() -> GetStatsStmt
{ GetStatsStmt(cornucopia_async :: private :: Stmt :: new("(
    SELECT 'songs' AS table_name, COUNT(*) as count
    FROM songs
)
UNION ALL
(
    SELECT 'beats' AS table_name, COUNT(*) as count
    FROM beats
)
UNION ALL
(
    SELECT 'covers' AS table_name, COUNT(*) as count
    FROM covers
)
UNION ALL
(
    SELECT 'lyrics' AS table_name, COUNT(*) as count
    FROM lyrics
)")) } pub
struct GetStatsStmt(cornucopia_async :: private :: Stmt) ; impl
GetStatsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> GetStatsQuery < 'a, C,
GetStats, 0 >
{
    GetStatsQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { GetStatsBorrowed { table_name : row.get(0),count : row.get(1),} }, mapper : | it | { <GetStats>::from(it) },
    }
} }pub fn get_genres_list() -> GetGenresListStmt
{ GetGenresListStmt(cornucopia_async :: private :: Stmt :: new("SELECT name from genres")) } pub
struct GetGenresListStmt(cornucopia_async :: private :: Stmt) ; impl
GetGenresListStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> StringQuery < 'a, C,
String, 0 >
{
    StringQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it.into() },
    }
} }pub fn get_moods_list() -> GetMoodsListStmt
{ GetMoodsListStmt(cornucopia_async :: private :: Stmt :: new("SELECT name from moods")) } pub
struct GetMoodsListStmt(cornucopia_async :: private :: Stmt) ; impl
GetMoodsListStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> StringQuery < 'a, C,
String, 0 >
{
    StringQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it.into() },
    }
} }pub fn get_songs() -> GetSongsStmt
{ GetSongsStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
    s.song_id,
    s.created_at,
    s.cover_url,
    s.name,
    s.author,
    s.likes,
    s.listenings,
    s.relevance_score,
    s.price,
    BOOL_OR(l.users_id = $1) AS is_user_liked
FROM available_songs s
LEFT JOIN likes l ON s.song_id = l.songs_id AND l.users_id = $1
WHERE
    ($2::varchar(6) IS NULL OR s.sex = $2::varchar(6))
AND (($3)::smallint[] IS NULL OR ($3)::smallint[] IS NOT NULL
    AND s.tempo BETWEEN (($3)::smallint[])[1] AND (($3)::smallint[])[2])
AND (($4)::musickey[] IS NULL OR s.key = ANY(($4)::musickey[]))
AND (($5)::text[] IS NULL OR s.primary_genre::text = ANY(($5)::text[]))
AND (($6)::text[] IS NULL OR s.vibes::text[] && ($6)::text[])
GROUP BY s.song_id, s.created_at, s.cover_url, s.name, s.author, s.likes, s.listenings, s.relevance_score, s.price
ORDER BY
    CASE WHEN $7 = 'top_wished' THEN s.likes END DESC NULLS LAST,
    CASE WHEN $7 = 'top_listened' THEN s.listenings END DESC NULLS LAST,
    CASE WHEN $7 = 'budget' THEN s.price END ASC NULLS LAST,
    CASE WHEN $7 = 'expensive' THEN s.price END DESC NULLS LAST,
    CASE WHEN $7 = 'new_first' THEN s.created_at END DESC NULLS LAST,
    CASE WHEN $7 = 'old_first' THEN s.created_at END ASC NULLS LAST,
    CASE WHEN $7 = 'relevance' THEN s.relevance_score END DESC
OFFSET $8
LIMIT $9")) } pub
struct GetSongsStmt(cornucopia_async :: private :: Stmt) ; impl
GetSongsStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::ArraySql<Item = i16>,T3 : cornucopia_async::ArraySql<Item = super::super::types::public::Musickey>,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::ArraySql<Item = T4>,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::ArraySql<Item = T6>,T8 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
user_id : & 'a Option<i32>,sex : & 'a Option<T1>,tempo : & 'a Option<T2>,key : & 'a Option<T3>,genre : & 'a Option<T5>,mood : & 'a Option<T7>,sort_by : & 'a T8,offset : & 'a i64,amount : & 'a i64,) -> GetSongsQuery < 'a, C,
GetSongs, 9 >
{
    GetSongsQuery
    {
        client, params : [user_id,sex,tempo,key,genre,mood,sort_by,offset,amount,], stmt : & mut self.0, extractor :
        | row | { GetSongsBorrowed { song_id : row.get(0),created_at : row.get(1),cover_url : row.get(2),name : row.get(3),author : row.get(4),likes : row.get(5),listenings : row.get(6),relevance_score : row.get(7),price : row.get(8),is_user_liked : row.get(9),} }, mapper : | it | { <GetSongs>::from(it) },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::ArraySql<Item = i16>,T3 : cornucopia_async::ArraySql<Item = super::super::types::public::Musickey>,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::ArraySql<Item = T4>,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::ArraySql<Item = T6>,T8 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, GetSongsParams < T1,T2,T3,T4,T5,T6,T7,T8,>, GetSongsQuery < 'a,
C, GetSongs, 9 >, C > for GetSongsStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    GetSongsParams < T1,T2,T3,T4,T5,T6,T7,T8,>) -> GetSongsQuery < 'a, C,
    GetSongs, 9 >
    { self.bind(client, & params.user_id,& params.sex,& params.tempo,& params.key,& params.genre,& params.mood,& params.sort_by,& params.offset,& params.amount,) }
}pub fn get_new_songs() -> GetNewSongsStmt
{ GetNewSongsStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
s.song_id,
s.created_at,
s.cover_url,
s.name,
s.author,
s.likes,
s.price,
BOOL_OR(l.users_id = $1) AS is_user_liked
FROM available_songs s
LEFT JOIN likes l ON s.song_id = l.songs_id AND l.users_id = $1
WHERE current_timestamp - s.created_at < '2 weeks'::interval
GROUP BY s.song_id, s.created_at, s.cover_url, s.name, s.author, s.likes, s.price
ORDER BY s.created_at DESC
LIMIT $2")) } pub
struct GetNewSongsStmt(cornucopia_async :: private :: Stmt) ; impl
GetNewSongsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
user_id : & 'a Option<i32>,amount : & 'a i64,) -> GetNewSongsQuery < 'a, C,
GetNewSongs, 2 >
{
    GetNewSongsQuery
    {
        client, params : [user_id,amount,], stmt : & mut self.0, extractor :
        | row | { GetNewSongsBorrowed { song_id : row.get(0),created_at : row.get(1),cover_url : row.get(2),name : row.get(3),author : row.get(4),likes : row.get(5),price : row.get(6),is_user_liked : row.get(7),} }, mapper : | it | { <GetNewSongs>::from(it) },
    }
} }impl < 'a, C : GenericClient, > cornucopia_async ::
Params < 'a, GetNewSongsParams < >, GetNewSongsQuery < 'a,
C, GetNewSongs, 2 >, C > for GetNewSongsStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    GetNewSongsParams < >) -> GetNewSongsQuery < 'a, C,
    GetNewSongs, 2 >
    { self.bind(client, & params.user_id,& params.amount,) }
}pub fn get_recommended_songs() -> GetRecommendedSongsStmt
{ GetRecommendedSongsStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
s.song_id,
s.created_at,
s.cover_url,
s.name,
s.author,
s.likes,
s.price,
BOOL_OR(l.users_id = $1) AS is_user_liked
FROM available_songs s
RIGHT JOIN (
    SELECT likes.songs_id
    FROM likes
    JOIN users ON likes.users_id = users.id
    JOIN users_groups ON users.id = users_groups.users_id
    JOIN groups ON users_groups.groups_id = groups.id
    WHERE songs_id IS NOT NULL AND groups.name = 'group.administrators'
) AS admin_likes
ON song_id = admin_likes.songs_id
LEFT JOIN likes l ON s.song_id = l.songs_id AND l.users_id = $1
WHERE current_timestamp - s.created_at < '1 month'::interval
GROUP BY s.song_id, s.created_at, s.cover_url, s.name, s.author, s.likes, s.price
ORDER BY s.created_at DESC
LIMIT $2")) } pub
struct GetRecommendedSongsStmt(cornucopia_async :: private :: Stmt) ; impl
GetRecommendedSongsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
user_id : & 'a Option<i32>,amount : & 'a i64,) -> GetRecommendedSongsQuery < 'a, C,
GetRecommendedSongs, 2 >
{
    GetRecommendedSongsQuery
    {
        client, params : [user_id,amount,], stmt : & mut self.0, extractor :
        | row | { GetRecommendedSongsBorrowed { song_id : row.get(0),created_at : row.get(1),cover_url : row.get(2),name : row.get(3),author : row.get(4),likes : row.get(5),price : row.get(6),is_user_liked : row.get(7),} }, mapper : | it | { <GetRecommendedSongs>::from(it) },
    }
} }impl < 'a, C : GenericClient, > cornucopia_async ::
Params < 'a, GetRecommendedSongsParams < >, GetRecommendedSongsQuery < 'a,
C, GetRecommendedSongs, 2 >, C > for GetRecommendedSongsStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    GetRecommendedSongsParams < >) -> GetRecommendedSongsQuery < 'a, C,
    GetRecommendedSongs, 2 >
    { self.bind(client, & params.user_id,& params.amount,) }
}}pub mod tests
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct SelectUserDataWithAvatarKey
{ pub id : i32,pub key : String,pub username : String,pub email : String,}pub struct SelectUserDataWithAvatarKeyBorrowed < 'a >
{ pub id : i32,pub key : &'a str,pub username : &'a str,pub email : &'a str,} impl < 'a > From < SelectUserDataWithAvatarKeyBorrowed <
'a >> for SelectUserDataWithAvatarKey
{
    fn
    from(SelectUserDataWithAvatarKeyBorrowed { id,key,username,email,} : SelectUserDataWithAvatarKeyBorrowed < 'a >)
    -> Self { Self { id,key: key.into(),username: username.into(),email: email.into(),} }
}pub struct SelectUserDataWithAvatarKeyQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> SelectUserDataWithAvatarKeyBorrowed,
    mapper : fn(SelectUserDataWithAvatarKeyBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > SelectUserDataWithAvatarKeyQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(SelectUserDataWithAvatarKeyBorrowed) -> R) -> SelectUserDataWithAvatarKeyQuery
    < 'a, C, R, N >
    {
        SelectUserDataWithAvatarKeyQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn select_user_data_with_avatar_key() -> SelectUserDataWithAvatarKeyStmt
{ SelectUserDataWithAvatarKeyStmt(cornucopia_async :: private :: Stmt :: new("SELECT users.id, objects.key, username, email
FROM users
JOIN objects
ON users.id = objects.avatar_users_id
WHERE users.username = $1")) } pub
struct SelectUserDataWithAvatarKeyStmt(cornucopia_async :: private :: Stmt) ; impl
SelectUserDataWithAvatarKeyStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
username : & 'a T1,) -> SelectUserDataWithAvatarKeyQuery < 'a, C,
SelectUserDataWithAvatarKey, 1 >
{
    SelectUserDataWithAvatarKeyQuery
    {
        client, params : [username,], stmt : & mut self.0, extractor :
        | row | { SelectUserDataWithAvatarKeyBorrowed { id : row.get(0),key : row.get(1),username : row.get(2),email : row.get(3),} }, mapper : | it | { <SelectUserDataWithAvatarKey>::from(it) },
    }
} }}pub mod user_access
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(Clone,Copy, Debug)] pub struct SetUserSettingsParams < > { pub inbox_messages : bool,pub order_messages : bool,pub order_updates : bool,pub id : i32,}#[derive(Clone,Copy, Debug)] pub struct SetSystemNotificationHaveBeenSeenParams < > { pub user_id : i32,pub system_notification_id : i32,}#[derive(Clone,Copy, Debug)] pub struct GetConversationByUserIdParams < > { pub first_user_id : i32,pub second_user_id : i32,}#[derive(Clone,Copy, Debug)] pub struct ListConversationByIdParams < > { pub conversation_id : i32,pub offset : i64,}#[derive(Clone,Copy, Debug)] pub struct AddParticipantsToConversationParams < > { pub conversation_id : i32,pub user1 : i32,pub user2 : i32,}#[derive( Debug)] pub struct InsertNewMessageParams < T1 : cornucopia_async::StringSql,> { pub conversation_id : i32,pub service_id : Option<i32>,pub user_id : i32,pub reply_message_id : Option<i32>,pub text : T1,}#[derive( Debug)] pub struct InsertMessageAttachmentParams < T1 : cornucopia_async::StringSql,> { pub key : T1,pub message_id : i32,}#[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)] pub struct GetUserSettings
{ pub inbox_messages : bool,pub order_messages : bool,pub order_updates : bool,}pub struct GetUserSettingsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetUserSettings,
    mapper : fn(GetUserSettings) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetUserSettingsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetUserSettings) -> R) -> GetUserSettingsQuery
    < 'a, C, R, N >
    {
        GetUserSettingsQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetUserSystemNotifications
{ pub id : i32,pub text : String,pub users_id : i32,pub created_at : time::OffsetDateTime,pub system_notifications_id : Option<i32>,}pub struct GetUserSystemNotificationsBorrowed < 'a >
{ pub id : i32,pub text : &'a str,pub users_id : i32,pub created_at : time::OffsetDateTime,pub system_notifications_id : Option<i32>,} impl < 'a > From < GetUserSystemNotificationsBorrowed <
'a >> for GetUserSystemNotifications
{
    fn
    from(GetUserSystemNotificationsBorrowed { id,text,users_id,created_at,system_notifications_id,} : GetUserSystemNotificationsBorrowed < 'a >)
    -> Self { Self { id,text: text.into(),users_id,created_at,system_notifications_id,} }
}pub struct GetUserSystemNotificationsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetUserSystemNotificationsBorrowed,
    mapper : fn(GetUserSystemNotificationsBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetUserSystemNotificationsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetUserSystemNotificationsBorrowed) -> R) -> GetUserSystemNotificationsQuery
    < 'a, C, R, N >
    {
        GetUserSystemNotificationsQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub struct Optioni32Query < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> Option<i32>,
    mapper : fn(Option<i32>) -> T,
} impl < 'a, C, T : 'a, const N : usize > Optioni32Query < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(Option<i32>) -> R) -> Optioni32Query
    < 'a, C, R, N >
    {
        Optioni32Query
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetConversationsEntries
{ pub conversation_id : i32,pub interlocutor : String,pub last_message_text : String,pub last_message_timestamp : time::OffsetDateTime,pub image_url : String,pub unread_messages_count : i64,}pub struct GetConversationsEntriesBorrowed < 'a >
{ pub conversation_id : i32,pub interlocutor : &'a str,pub last_message_text : &'a str,pub last_message_timestamp : time::OffsetDateTime,pub image_url : &'a str,pub unread_messages_count : i64,} impl < 'a > From < GetConversationsEntriesBorrowed <
'a >> for GetConversationsEntries
{
    fn
    from(GetConversationsEntriesBorrowed { conversation_id,interlocutor,last_message_text,last_message_timestamp,image_url,unread_messages_count,} : GetConversationsEntriesBorrowed < 'a >)
    -> Self { Self { conversation_id,interlocutor: interlocutor.into(),last_message_text: last_message_text.into(),last_message_timestamp,image_url: image_url.into(),unread_messages_count,} }
}pub struct GetConversationsEntriesQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetConversationsEntriesBorrowed,
    mapper : fn(GetConversationsEntriesBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetConversationsEntriesQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetConversationsEntriesBorrowed) -> R) -> GetConversationsEntriesQuery
    < 'a, C, R, N >
    {
        GetConversationsEntriesQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct ListConversationById
{ pub conversation_id : i32,pub participant_user_id : i32,pub participant_username : String,pub participant_avatar_key : String,pub message_id : Option<i32>,pub message_text : Option<String>,pub message_created_at : Option<time::OffsetDateTime>,pub message_updated_at : Option<time::OffsetDateTime>,pub reply_message_id : Option<i32>,pub message_attachments : Option<Vec<String>>,pub service_id : Option<i32>,pub service_name : Option<String>,pub service_cover_key : Option<String>,pub offer_id : Option<i32>,pub offer_text : Option<String>,pub offer_price : Option<rust_decimal::Decimal>,pub offer_delivery_date : Option<time::OffsetDateTime>,pub offer_free_revisions : Option<i32>,pub offer_revision_price : Option<rust_decimal::Decimal>,}pub struct ListConversationByIdBorrowed < 'a >
{ pub conversation_id : i32,pub participant_user_id : i32,pub participant_username : &'a str,pub participant_avatar_key : &'a str,pub message_id : Option<i32>,pub message_text : Option<&'a str>,pub message_created_at : Option<time::OffsetDateTime>,pub message_updated_at : Option<time::OffsetDateTime>,pub reply_message_id : Option<i32>,pub message_attachments : Option<cornucopia_async::ArrayIterator<'a, &'a str>>,pub service_id : Option<i32>,pub service_name : Option<&'a str>,pub service_cover_key : Option<&'a str>,pub offer_id : Option<i32>,pub offer_text : Option<&'a str>,pub offer_price : Option<rust_decimal::Decimal>,pub offer_delivery_date : Option<time::OffsetDateTime>,pub offer_free_revisions : Option<i32>,pub offer_revision_price : Option<rust_decimal::Decimal>,} impl < 'a > From < ListConversationByIdBorrowed <
'a >> for ListConversationById
{
    fn
    from(ListConversationByIdBorrowed { conversation_id,participant_user_id,participant_username,participant_avatar_key,message_id,message_text,message_created_at,message_updated_at,reply_message_id,message_attachments,service_id,service_name,service_cover_key,offer_id,offer_text,offer_price,offer_delivery_date,offer_free_revisions,offer_revision_price,} : ListConversationByIdBorrowed < 'a >)
    -> Self { Self { conversation_id,participant_user_id,participant_username: participant_username.into(),participant_avatar_key: participant_avatar_key.into(),message_id,message_text: message_text.map(|v| v.into()),message_created_at,message_updated_at,reply_message_id,message_attachments: message_attachments.map(|v| v.map(|v| v.into()).collect()),service_id,service_name: service_name.map(|v| v.into()),service_cover_key: service_cover_key.map(|v| v.into()),offer_id,offer_text: offer_text.map(|v| v.into()),offer_price,offer_delivery_date,offer_free_revisions,offer_revision_price,} }
}pub struct ListConversationByIdQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> ListConversationByIdBorrowed,
    mapper : fn(ListConversationByIdBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > ListConversationByIdQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(ListConversationByIdBorrowed) -> R) -> ListConversationByIdQuery
    < 'a, C, R, N >
    {
        ListConversationByIdQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub struct I32Query < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> i32,
    mapper : fn(i32) -> T,
} impl < 'a, C, T : 'a, const N : usize > I32Query < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(i32) -> R) -> I32Query
    < 'a, C, R, N >
    {
        I32Query
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn get_user_settings() -> GetUserSettingsStmt
{ GetUserSettingsStmt(cornucopia_async :: private :: Stmt :: new("SELECT inbox_messages, order_messages, order_updates
FROM user_settings
JOIN users
ON users.user_settings_id = user_settings.id
WHERE users.id = $1")) } pub
struct GetUserSettingsStmt(cornucopia_async :: private :: Stmt) ; impl
GetUserSettingsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
user_id : & 'a i32,) -> GetUserSettingsQuery < 'a, C,
GetUserSettings, 1 >
{
    GetUserSettingsQuery
    {
        client, params : [user_id,], stmt : & mut self.0, extractor :
        | row | { GetUserSettings { inbox_messages : row.get(0),order_messages : row.get(1),order_updates : row.get(2),} }, mapper : | it | { <GetUserSettings>::from(it) },
    }
} }pub fn set_user_settings() -> SetUserSettingsStmt
{ SetUserSettingsStmt(cornucopia_async :: private :: Stmt :: new("UPDATE user_settings
SET inbox_messages = $1, order_messages = $2, order_updates = $3
WHERE id = (
    SELECT user_settings_id
    FROM users
    WHERE id = $4
)")) } pub
struct SetUserSettingsStmt(cornucopia_async :: private :: Stmt) ; impl
SetUserSettingsStmt { pub async fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
inbox_messages : & 'a bool,order_messages : & 'a bool,order_updates : & 'a bool,id : & 'a i32,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [inbox_messages,order_messages,order_updates,id,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, >
cornucopia_async :: Params < 'a, SetUserSettingsParams < >, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for SetUserSettingsStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    SetUserSettingsParams < >) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.inbox_messages,& params.order_messages,& params.order_updates,& params.id,) ) }
}pub fn get_user_system_notifications() -> GetUserSystemNotificationsStmt
{ GetUserSystemNotificationsStmt(cornucopia_async :: private :: Stmt :: new("SELECT s.id, s.text, s.users_id, s.created_at, views.system_notifications_id
FROM system_notifications s
LEFT JOIN views
ON views.system_notifications_id = s.id
RIGHT JOIN users
ON users.id = s.users_id
ORDER BY s.created_at DESC")) } pub
struct GetUserSystemNotificationsStmt(cornucopia_async :: private :: Stmt) ; impl
GetUserSystemNotificationsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> GetUserSystemNotificationsQuery < 'a, C,
GetUserSystemNotifications, 0 >
{
    GetUserSystemNotificationsQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { GetUserSystemNotificationsBorrowed { id : row.get(0),text : row.get(1),users_id : row.get(2),created_at : row.get(3),system_notifications_id : row.get(4),} }, mapper : | it | { <GetUserSystemNotifications>::from(it) },
    }
} }pub fn set_system_notification_have_been_seen() -> SetSystemNotificationHaveBeenSeenStmt
{ SetSystemNotificationHaveBeenSeenStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO views (users_id, system_notifications_id)
VALUES ($1, $2)")) } pub
struct SetSystemNotificationHaveBeenSeenStmt(cornucopia_async :: private :: Stmt) ; impl
SetSystemNotificationHaveBeenSeenStmt { pub async fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
user_id : & 'a i32,system_notification_id : & 'a i32,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [user_id,system_notification_id,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, >
cornucopia_async :: Params < 'a, SetSystemNotificationHaveBeenSeenParams < >, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for SetSystemNotificationHaveBeenSeenStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    SetSystemNotificationHaveBeenSeenParams < >) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.user_id,& params.system_notification_id,) ) }
}pub fn get_conversation_by_user_id() -> GetConversationByUserIdStmt
{ GetConversationByUserIdStmt(cornucopia_async :: private :: Stmt :: new("SELECT c.id AS conversations_id
FROM conversations c
JOIN participants p1 ON c.id = p1.conversations_id AND p1.users_id = $1
JOIN participants p2 ON c.id = p2.conversations_id AND p2.users_id = $2")) } pub
struct GetConversationByUserIdStmt(cornucopia_async :: private :: Stmt) ; impl
GetConversationByUserIdStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
first_user_id : & 'a i32,second_user_id : & 'a i32,) -> Optioni32Query < 'a, C,
Option<i32>, 2 >
{
    Optioni32Query
    {
        client, params : [first_user_id,second_user_id,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }impl < 'a, C : GenericClient, > cornucopia_async ::
Params < 'a, GetConversationByUserIdParams < >, Optioni32Query < 'a,
C, Option<i32>, 2 >, C > for GetConversationByUserIdStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    GetConversationByUserIdParams < >) -> Optioni32Query < 'a, C,
    Option<i32>, 2 >
    { self.bind(client, & params.first_user_id,& params.second_user_id,) }
}pub fn get_conversations_entries() -> GetConversationsEntriesStmt
{ GetConversationsEntriesStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
    conversations.id AS conversation_id,
    interlocutor.username AS interlocutor,
    last_message.text AS last_message_text,
    last_message.created_at AS last_message_timestamp,
    interlocutor_avatar.key AS image_url,
    (SELECT COUNT(*) 
        FROM messages 
        WHERE messages.conversations_id = conversations.id 
        AND messages.id NOT IN (SELECT messages_id FROM views WHERE users_id = $1)
    ) AS unread_messages_count
FROM 
    conversations
JOIN 
    participants ON participants.conversations_id = conversations.id
JOIN 
    users AS interlocutor ON participants.users_id = interlocutor.id AND interlocutor.id != $1
LEFT JOIN 
    objects AS interlocutor_avatar ON interlocutor_avatar.avatar_users_id = interlocutor.id
LEFT JOIN LATERAL
    (SELECT m1.*
        FROM messages m1
        WHERE m1.conversations_id = conversations.id
        ORDER BY m1.created_at DESC
        LIMIT 1
    ) last_message ON TRUE
WHERE 
    conversations.id IN (SELECT conversations_id FROM participants WHERE users_id = $1)")) } pub
struct GetConversationsEntriesStmt(cornucopia_async :: private :: Stmt) ; impl
GetConversationsEntriesStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
user_id : & 'a i32,) -> GetConversationsEntriesQuery < 'a, C,
GetConversationsEntries, 1 >
{
    GetConversationsEntriesQuery
    {
        client, params : [user_id,], stmt : & mut self.0, extractor :
        | row | { GetConversationsEntriesBorrowed { conversation_id : row.get(0),interlocutor : row.get(1),last_message_text : row.get(2),last_message_timestamp : row.get(3),image_url : row.get(4),unread_messages_count : row.get(5),} }, mapper : | it | { <GetConversationsEntries>::from(it) },
    }
} }pub fn list_conversation_by_id() -> ListConversationByIdStmt
{ ListConversationByIdStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
    conv.id as conversation_id,
    part.users_id as participant_user_id,
    usr.username as participant_username,
    obj.key as participant_avatar_key,
    msg.id as message_id,
    msg.text as message_text,
    msg.created_at as message_created_at,
    msg.updated_at as message_updated_at,
    msg.messages_id as reply_message_id,
    ARRAY_AGG(DISTINCT obj3.key) FILTER (WHERE obj3.key IS NOT NULL) as message_attachments,
    serv.id as service_id,
    serv.name as service_name,
    obj2.key as service_cover_key,
    off.id as offer_id,
    off.text as offer_text,
    off.price as offer_price,
    off.delivery_date as offer_delivery_date,
    off.free_revisions as offer_free_revisions,
    off.revision_price as offer_revision_price
FROM 
    conversations conv
LEFT JOIN participants part ON part.conversations_id = conv.id
LEFT JOIN users usr ON part.users_id = usr.id
LEFT JOIN messages msg ON msg.conversations_id = conv.id AND msg.users_id = part.users_id
LEFT JOIN offers off ON off.conversations_id = conv.id
LEFT JOIN services serv ON serv.id = COALESCE(msg.services_id, off.services_id)
LEFT JOIN objects obj ON obj.avatar_users_id = usr.id
LEFT JOIN objects obj2 ON obj.cover_services_id = serv.id
LEFT JOIN objects obj3 ON obj.message_attachment = msg.id
WHERE 
    conv.id = $1
GROUP BY 
    msg.id, conv.id, part.users_id, usr.username, obj.key, serv.id, serv.name, obj2.key, off.id, off.text, off.price, off.delivery_date, off.free_revisions, off.revision_price
ORDER BY 
    msg.created_at ASC, 
    off.created_at ASC
OFFSET $2
LIMIT 30")) } pub
struct ListConversationByIdStmt(cornucopia_async :: private :: Stmt) ; impl
ListConversationByIdStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
conversation_id : & 'a i32,offset : & 'a i64,) -> ListConversationByIdQuery < 'a, C,
ListConversationById, 2 >
{
    ListConversationByIdQuery
    {
        client, params : [conversation_id,offset,], stmt : & mut self.0, extractor :
        | row | { ListConversationByIdBorrowed { conversation_id : row.get(0),participant_user_id : row.get(1),participant_username : row.get(2),participant_avatar_key : row.get(3),message_id : row.get(4),message_text : row.get(5),message_created_at : row.get(6),message_updated_at : row.get(7),reply_message_id : row.get(8),message_attachments : row.get(9),service_id : row.get(10),service_name : row.get(11),service_cover_key : row.get(12),offer_id : row.get(13),offer_text : row.get(14),offer_price : row.get(15),offer_delivery_date : row.get(16),offer_free_revisions : row.get(17),offer_revision_price : row.get(18),} }, mapper : | it | { <ListConversationById>::from(it) },
    }
} }impl < 'a, C : GenericClient, > cornucopia_async ::
Params < 'a, ListConversationByIdParams < >, ListConversationByIdQuery < 'a,
C, ListConversationById, 2 >, C > for ListConversationByIdStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    ListConversationByIdParams < >) -> ListConversationByIdQuery < 'a, C,
    ListConversationById, 2 >
    { self.bind(client, & params.conversation_id,& params.offset,) }
}pub fn create_new_conversation() -> CreateNewConversationStmt
{ CreateNewConversationStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO conversations VALUES (DEFAULT) returning id")) } pub
struct CreateNewConversationStmt(cornucopia_async :: private :: Stmt) ; impl
CreateNewConversationStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> I32Query < 'a, C,
i32, 0 >
{
    I32Query
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }pub fn add_participants_to_conversation() -> AddParticipantsToConversationStmt
{ AddParticipantsToConversationStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO participants (conversations_id, users_id)
VALUES
    ($1, $2),
    ($1, $3)")) } pub
struct AddParticipantsToConversationStmt(cornucopia_async :: private :: Stmt) ; impl
AddParticipantsToConversationStmt { pub async fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
conversation_id : & 'a i32,user1 : & 'a i32,user2 : & 'a i32,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [conversation_id,user1,user2,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, >
cornucopia_async :: Params < 'a, AddParticipantsToConversationParams < >, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for AddParticipantsToConversationStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    AddParticipantsToConversationParams < >) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.conversation_id,& params.user1,& params.user2,) ) }
}pub fn insert_new_message() -> InsertNewMessageStmt
{ InsertNewMessageStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO messages (conversations_id, services_id, users_id, messages_id, text)
VALUES ($1, $2, $3, $4, $5) returning id")) } pub
struct InsertNewMessageStmt(cornucopia_async :: private :: Stmt) ; impl
InsertNewMessageStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
conversation_id : & 'a i32,service_id : & 'a Option<i32>,user_id : & 'a i32,reply_message_id : & 'a Option<i32>,text : & 'a T1,) -> I32Query < 'a, C,
i32, 5 >
{
    I32Query
    {
        client, params : [conversation_id,service_id,user_id,reply_message_id,text,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, InsertNewMessageParams < T1,>, I32Query < 'a,
C, i32, 5 >, C > for InsertNewMessageStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertNewMessageParams < T1,>) -> I32Query < 'a, C,
    i32, 5 >
    { self.bind(client, & params.conversation_id,& params.service_id,& params.user_id,& params.reply_message_id,& params.text,) }
}pub fn insert_message_attachment() -> InsertMessageAttachmentStmt
{ InsertMessageAttachmentStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO objects (key, object_type, message_attachment)
VALUES ($1, 'attachment', $2)")) } pub
struct InsertMessageAttachmentStmt(cornucopia_async :: private :: Stmt) ; impl
InsertMessageAttachmentStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
key : & 'a T1,message_id : & 'a i32,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [key,message_id,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, InsertMessageAttachmentParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for InsertMessageAttachmentStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertMessageAttachmentParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.key,& params.message_id,) ) }
}}pub mod user_auth_queries
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CheckIfUserExistsAlreadyParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> { pub email : T1,pub username : T2,}#[derive( Debug)] pub struct InsertNewUserParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,> { pub user_settings_id : i32,pub username : T1,pub email : T2,pub password_hash : T3,}#[derive( Debug)] pub struct StoreUserPermissionParams < T1 : cornucopia_async::StringSql,> { pub user_id : i32,pub permission : T1,}#[derive( Debug)] pub struct InsertUserAvatarImageParams < T1 : cornucopia_async::StringSql,> { pub key : T1,pub users_id : i32,}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetAuthUserDataByUsername
{ pub id : i32,pub username : String,pub password_hash : String,}pub struct GetAuthUserDataByUsernameBorrowed < 'a >
{ pub id : i32,pub username : &'a str,pub password_hash : &'a str,} impl < 'a > From < GetAuthUserDataByUsernameBorrowed <
'a >> for GetAuthUserDataByUsername
{
    fn
    from(GetAuthUserDataByUsernameBorrowed { id,username,password_hash,} : GetAuthUserDataByUsernameBorrowed < 'a >)
    -> Self { Self { id,username: username.into(),password_hash: password_hash.into(),} }
}pub struct GetAuthUserDataByUsernameQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetAuthUserDataByUsernameBorrowed,
    mapper : fn(GetAuthUserDataByUsernameBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetAuthUserDataByUsernameQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetAuthUserDataByUsernameBorrowed) -> R) -> GetAuthUserDataByUsernameQuery
    < 'a, C, R, N >
    {
        GetAuthUserDataByUsernameQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetAuthUserDataById
{ pub id : i32,pub username : String,pub password_hash : String,}pub struct GetAuthUserDataByIdBorrowed < 'a >
{ pub id : i32,pub username : &'a str,pub password_hash : &'a str,} impl < 'a > From < GetAuthUserDataByIdBorrowed <
'a >> for GetAuthUserDataById
{
    fn
    from(GetAuthUserDataByIdBorrowed { id,username,password_hash,} : GetAuthUserDataByIdBorrowed < 'a >)
    -> Self { Self { id,username: username.into(),password_hash: password_hash.into(),} }
}pub struct GetAuthUserDataByIdQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetAuthUserDataByIdBorrowed,
    mapper : fn(GetAuthUserDataByIdBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetAuthUserDataByIdQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetAuthUserDataByIdBorrowed) -> R) -> GetAuthUserDataByIdQuery
    < 'a, C, R, N >
    {
        GetAuthUserDataByIdQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub struct StringQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> & str,
    mapper : fn(& str) -> T,
} impl < 'a, C, T : 'a, const N : usize > StringQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(& str) -> R) -> StringQuery
    < 'a, C, R, N >
    {
        StringQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)] pub struct GetAdminSignupToken
{ pub token : uuid::Uuid,pub used : bool,}pub struct GetAdminSignupTokenQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetAdminSignupToken,
    mapper : fn(GetAdminSignupToken) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetAdminSignupTokenQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetAdminSignupToken) -> R) -> GetAdminSignupTokenQuery
    < 'a, C, R, N >
    {
        GetAdminSignupTokenQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub struct I32Query < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> i32,
    mapper : fn(i32) -> T,
} impl < 'a, C, T : 'a, const N : usize > I32Query < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(i32) -> R) -> I32Query
    < 'a, C, R, N >
    {
        I32Query
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn get_auth_user_data_by_username() -> GetAuthUserDataByUsernameStmt
{ GetAuthUserDataByUsernameStmt(cornucopia_async :: private :: Stmt :: new("SELECT id, username, password_hash
FROM users
WHERE username = $1")) } pub
struct GetAuthUserDataByUsernameStmt(cornucopia_async :: private :: Stmt) ; impl
GetAuthUserDataByUsernameStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
username : & 'a T1,) -> GetAuthUserDataByUsernameQuery < 'a, C,
GetAuthUserDataByUsername, 1 >
{
    GetAuthUserDataByUsernameQuery
    {
        client, params : [username,], stmt : & mut self.0, extractor :
        | row | { GetAuthUserDataByUsernameBorrowed { id : row.get(0),username : row.get(1),password_hash : row.get(2),} }, mapper : | it | { <GetAuthUserDataByUsername>::from(it) },
    }
} }pub fn get_auth_user_data_by_id() -> GetAuthUserDataByIdStmt
{ GetAuthUserDataByIdStmt(cornucopia_async :: private :: Stmt :: new("SELECT id, username, password_hash
FROM users
WHERE id = $1")) } pub
struct GetAuthUserDataByIdStmt(cornucopia_async :: private :: Stmt) ; impl
GetAuthUserDataByIdStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
id : & 'a i32,) -> GetAuthUserDataByIdQuery < 'a, C,
GetAuthUserDataById, 1 >
{
    GetAuthUserDataByIdQuery
    {
        client, params : [id,], stmt : & mut self.0, extractor :
        | row | { GetAuthUserDataByIdBorrowed { id : row.get(0),username : row.get(1),password_hash : row.get(2),} }, mapper : | it | { <GetAuthUserDataById>::from(it) },
    }
} }pub fn get_user_permissions() -> GetUserPermissionsStmt
{ GetUserPermissionsStmt(cornucopia_async :: private :: Stmt :: new("SELECT DISTINCT permissions.name
FROM users
JOIN users_groups
ON users.id = users_groups.users_id
JOIN groups_permissions
ON users_groups.groups_id = groups_permissions.groups_id
JOIN permissions
ON groups_permissions.permissions_id = permissions.id
WHERE users.id = $1")) } pub
struct GetUserPermissionsStmt(cornucopia_async :: private :: Stmt) ; impl
GetUserPermissionsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
user_id : & 'a i32,) -> StringQuery < 'a, C,
String, 1 >
{
    StringQuery
    {
        client, params : [user_id,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it.into() },
    }
} }pub fn insert_a_new_admin_signup_token() -> InsertANewAdminSignupTokenStmt
{ InsertANewAdminSignupTokenStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO admin_signup_tokens (token)
VALUES ($1)")) } pub
struct InsertANewAdminSignupTokenStmt(cornucopia_async :: private :: Stmt) ; impl
InsertANewAdminSignupTokenStmt { pub async fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
token : & 'a uuid::Uuid,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [token,]) .await
} }pub fn get_admin_signup_token() -> GetAdminSignupTokenStmt
{ GetAdminSignupTokenStmt(cornucopia_async :: private :: Stmt :: new("SELECT token, used
FROM admin_signup_tokens
WHERE token = $1")) } pub
struct GetAdminSignupTokenStmt(cornucopia_async :: private :: Stmt) ; impl
GetAdminSignupTokenStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
token : & 'a uuid::Uuid,) -> GetAdminSignupTokenQuery < 'a, C,
GetAdminSignupToken, 1 >
{
    GetAdminSignupTokenQuery
    {
        client, params : [token,], stmt : & mut self.0, extractor :
        | row | { GetAdminSignupToken { token : row.get(0),used : row.get(1),} }, mapper : | it | { <GetAdminSignupToken>::from(it) },
    }
} }pub fn use_admin_signup_token() -> UseAdminSignupTokenStmt
{ UseAdminSignupTokenStmt(cornucopia_async :: private :: Stmt :: new("UPDATE admin_signup_tokens
SET used = TRUE
WHERE token = $1")) } pub
struct UseAdminSignupTokenStmt(cornucopia_async :: private :: Stmt) ; impl
UseAdminSignupTokenStmt { pub async fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
token : & 'a uuid::Uuid,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [token,]) .await
} }pub fn check_if_user_exists_already() -> CheckIfUserExistsAlreadyStmt
{ CheckIfUserExistsAlreadyStmt(cornucopia_async :: private :: Stmt :: new("SELECT id FROM users
WHERE email = $1 OR username = $2")) } pub
struct CheckIfUserExistsAlreadyStmt(cornucopia_async :: private :: Stmt) ; impl
CheckIfUserExistsAlreadyStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
email : & 'a T1,username : & 'a T2,) -> I32Query < 'a, C,
i32, 2 >
{
    I32Query
    {
        client, params : [email,username,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, CheckIfUserExistsAlreadyParams < T1,T2,>, I32Query < 'a,
C, i32, 2 >, C > for CheckIfUserExistsAlreadyStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    CheckIfUserExistsAlreadyParams < T1,T2,>) -> I32Query < 'a, C,
    i32, 2 >
    { self.bind(client, & params.email,& params.username,) }
}pub fn insert_new_user_settings() -> InsertNewUserSettingsStmt
{ InsertNewUserSettingsStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO user_settings DEFAULT VALUES returning id")) } pub
struct InsertNewUserSettingsStmt(cornucopia_async :: private :: Stmt) ; impl
InsertNewUserSettingsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> I32Query < 'a, C,
i32, 0 >
{
    I32Query
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }pub fn insert_new_user() -> InsertNewUserStmt
{ InsertNewUserStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO users
(user_settings_id, username, bio, email, password_hash, status)
VALUES ($1, $2, NULL, $3, $4, NULL) returning id")) } pub
struct InsertNewUserStmt(cornucopia_async :: private :: Stmt) ; impl
InsertNewUserStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
user_settings_id : & 'a i32,username : & 'a T1,email : & 'a T2,password_hash : & 'a T3,) -> I32Query < 'a, C,
i32, 4 >
{
    I32Query
    {
        client, params : [user_settings_id,username,email,password_hash,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, InsertNewUserParams < T1,T2,T3,>, I32Query < 'a,
C, i32, 4 >, C > for InsertNewUserStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertNewUserParams < T1,T2,T3,>) -> I32Query < 'a, C,
    i32, 4 >
    { self.bind(client, & params.user_settings_id,& params.username,& params.email,& params.password_hash,) }
}pub fn store_user_permission() -> StoreUserPermissionStmt
{ StoreUserPermissionStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO users_groups (users_id, groups_id)
VALUES (
    $1,
    (SELECT id FROM groups WHERE name = $2)
)")) } pub
struct StoreUserPermissionStmt(cornucopia_async :: private :: Stmt) ; impl
StoreUserPermissionStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
user_id : & 'a i32,permission : & 'a T1,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [user_id,permission,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, StoreUserPermissionParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for StoreUserPermissionStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    StoreUserPermissionParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.user_id,& params.permission,) ) }
}pub fn insert_user_avatar_image() -> InsertUserAvatarImageStmt
{ InsertUserAvatarImageStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO objects
(key, object_type, avatar_users_id)
VALUES ($1, 'image', $2)")) } pub
struct InsertUserAvatarImageStmt(cornucopia_async :: private :: Stmt) ; impl
InsertUserAvatarImageStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
key : & 'a T1,users_id : & 'a i32,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [key,users_id,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, InsertUserAvatarImageParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for InsertUserAvatarImageStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertUserAvatarImageParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.key,& params.users_id,) ) }
}}}