// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {
    pub mod public {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum Userrole {
            creator,
            consumer,
            fullstack,
        }
        impl<'a> postgres_types::ToSql for Userrole {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<
                postgres_types::IsNull,
                Box<dyn std::error::Error + Sync + Send>,
            > {
                let s = match *self {
                    Userrole::creator => "creator",
                    Userrole::consumer => "consumer",
                    Userrole::fullstack => "fullstack",
                };
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "userrole" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "creator" => true,
                            "consumer" => true,
                            "fullstack" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<
                postgres_types::IsNull,
                Box<dyn std::error::Error + Sync + Send>,
            > {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        impl<'a> postgres_types::FromSql<'a> for Userrole {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<Userrole, Box<dyn std::error::Error + Sync + Send>>
            {
                match std::str::from_utf8(buf)? {
                    "creator" => Ok(Userrole::creator),
                    "consumer" => Ok(Userrole::consumer),
                    "fullstack" => Ok(Userrole::fullstack),
                    s => Result::Err(Into::into(format!(
                        "invalid variant `{}`",
                        s
                    ))),
                }
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "userrole" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "creator" => true,
                            "consumer" => true,
                            "fullstack" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
        }
    }
}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod open_access {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetStats {
            pub table_name: String,
            pub count: i64,
        }
        pub struct GetStatsBorrowed<'a> {
            pub table_name: &'a str,
            pub count: i64,
        }
        impl<'a> From<GetStatsBorrowed<'a>> for GetStats {
            fn from(
                GetStatsBorrowed { table_name, count }: GetStatsBorrowed<'a>,
            ) -> Self {
                Self {
                    table_name: table_name.into(),
                    count,
                }
            }
        }
        pub struct GetStatsQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetStatsBorrowed,
            mapper: fn(GetStatsBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetStatsQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetStatsBorrowed) -> R,
            ) -> GetStatsQuery<'a, C, R, N> {
                GetStatsQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(
                        stmt,
                        cornucopia_async::private::slice_iter(&self.params),
                    )
                    .await?
                    .map(move |res| {
                        res.map(|row| (self.mapper)((self.extractor)(&row)))
                    })
                    .into_stream();
                Ok(it)
            }
        }
        pub fn get_stats() -> GetStatsStmt {
            GetStatsStmt(cornucopia_async::private::Stmt::new(
                "(
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
)",
            ))
        }
        pub struct GetStatsStmt(cornucopia_async::private::Stmt);
        impl GetStatsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> GetStatsQuery<'a, C, GetStats, 0> {
                GetStatsQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| GetStatsBorrowed {
                        table_name: row.get(0),
                        count: row.get(1),
                    },
                    mapper: |it| <GetStats>::from(it),
                }
            }
        }
    }
    pub mod tests {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectUserDataWithAvatarKey {
            pub key: String,
            pub username: String,
            pub email: String,
            pub role: super::super::types::public::Userrole,
        }
        pub struct SelectUserDataWithAvatarKeyBorrowed<'a> {
            pub key: &'a str,
            pub username: &'a str,
            pub email: &'a str,
            pub role: super::super::types::public::Userrole,
        }
        impl<'a> From<SelectUserDataWithAvatarKeyBorrowed<'a>>
            for SelectUserDataWithAvatarKey
        {
            fn from(
                SelectUserDataWithAvatarKeyBorrowed {
                    key,
                    username,
                    email,
                    role,
                }: SelectUserDataWithAvatarKeyBorrowed<'a>,
            ) -> Self {
                Self {
                    key: key.into(),
                    username: username.into(),
                    email: email.into(),
                    role,
                }
            }
        }
        pub struct SelectUserDataWithAvatarKeyQuery<
            'a,
            C: GenericClient,
            T,
            const N: usize,
        > {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor:
                fn(&tokio_postgres::Row) -> SelectUserDataWithAvatarKeyBorrowed,
            mapper: fn(SelectUserDataWithAvatarKeyBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectUserDataWithAvatarKeyQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectUserDataWithAvatarKeyBorrowed) -> R,
            ) -> SelectUserDataWithAvatarKeyQuery<'a, C, R, N> {
                SelectUserDataWithAvatarKeyQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(
                        stmt,
                        cornucopia_async::private::slice_iter(&self.params),
                    )
                    .await?
                    .map(move |res| {
                        res.map(|row| (self.mapper)((self.extractor)(&row)))
                    })
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_user_data_with_avatar_key(
        ) -> SelectUserDataWithAvatarKeyStmt {
            SelectUserDataWithAvatarKeyStmt(
                cornucopia_async::private::Stmt::new(
                    "SELECT objects.key, username, email, role
FROM users
JOIN objects
ON users.id = objects.avatar_users_id",
                ),
            )
        }
        pub struct SelectUserDataWithAvatarKeyStmt(
            cornucopia_async::private::Stmt,
        );
        impl SelectUserDataWithAvatarKeyStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> SelectUserDataWithAvatarKeyQuery<
                'a,
                C,
                SelectUserDataWithAvatarKey,
                0,
            > {
                SelectUserDataWithAvatarKeyQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| SelectUserDataWithAvatarKeyBorrowed {
                        key: row.get(0),
                        username: row.get(1),
                        email: row.get(2),
                        role: row.get(3),
                    },
                    mapper: |it| <SelectUserDataWithAvatarKey>::from(it),
                }
            }
        }
    }
    pub mod user_auth_queries {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct CheckIfUserExistsAlreadyParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub email: T1,
            pub username: T2,
        }
        #[derive(Debug)]
        pub struct InsertNewUserParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
        > {
            pub user_settings_id: i32,
            pub username: T1,
            pub email: T2,
            pub password_hash: T3,
            pub role: super::super::types::public::Userrole,
        }
        #[derive(Debug)]
        pub struct InsertUserImageParams<T1: cornucopia_async::StringSql> {
            pub key: T1,
            pub users_id: i32,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAuthUserDataByUsername {
            pub id: i32,
            pub username: String,
            pub password_hash: String,
        }
        pub struct GetAuthUserDataByUsernameBorrowed<'a> {
            pub id: i32,
            pub username: &'a str,
            pub password_hash: &'a str,
        }
        impl<'a> From<GetAuthUserDataByUsernameBorrowed<'a>>
            for GetAuthUserDataByUsername
        {
            fn from(
                GetAuthUserDataByUsernameBorrowed {
                    id,
                    username,
                    password_hash,
                }: GetAuthUserDataByUsernameBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    username: username.into(),
                    password_hash: password_hash.into(),
                }
            }
        }
        pub struct GetAuthUserDataByUsernameQuery<
            'a,
            C: GenericClient,
            T,
            const N: usize,
        > {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor:
                fn(&tokio_postgres::Row) -> GetAuthUserDataByUsernameBorrowed,
            mapper: fn(GetAuthUserDataByUsernameBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetAuthUserDataByUsernameQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetAuthUserDataByUsernameBorrowed) -> R,
            ) -> GetAuthUserDataByUsernameQuery<'a, C, R, N> {
                GetAuthUserDataByUsernameQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(
                        stmt,
                        cornucopia_async::private::slice_iter(&self.params),
                    )
                    .await?
                    .map(move |res| {
                        res.map(|row| (self.mapper)((self.extractor)(&row)))
                    })
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAuthUserDataById {
            pub id: i32,
            pub username: String,
            pub password_hash: String,
        }
        pub struct GetAuthUserDataByIdBorrowed<'a> {
            pub id: i32,
            pub username: &'a str,
            pub password_hash: &'a str,
        }
        impl<'a> From<GetAuthUserDataByIdBorrowed<'a>> for GetAuthUserDataById {
            fn from(
                GetAuthUserDataByIdBorrowed {
                    id,
                    username,
                    password_hash,
                }: GetAuthUserDataByIdBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    username: username.into(),
                    password_hash: password_hash.into(),
                }
            }
        }
        pub struct GetAuthUserDataByIdQuery<
            'a,
            C: GenericClient,
            T,
            const N: usize,
        > {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetAuthUserDataByIdBorrowed,
            mapper: fn(GetAuthUserDataByIdBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetAuthUserDataByIdQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetAuthUserDataByIdBorrowed) -> R,
            ) -> GetAuthUserDataByIdQuery<'a, C, R, N> {
                GetAuthUserDataByIdQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(
                        stmt,
                        cornucopia_async::private::slice_iter(&self.params),
                    )
                    .await?
                    .map(move |res| {
                        res.map(|row| (self.mapper)((self.extractor)(&row)))
                    })
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> i32,
            mapper: fn(i32) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> I32Query<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(i32) -> R) -> I32Query<'a, C, R, N> {
                I32Query {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(
                        stmt,
                        cornucopia_async::private::slice_iter(&self.params),
                    )
                    .await?
                    .map(move |res| {
                        res.map(|row| (self.mapper)((self.extractor)(&row)))
                    })
                    .into_stream();
                Ok(it)
            }
        }
        pub fn get_auth_user_data_by_username() -> GetAuthUserDataByUsernameStmt
        {
            GetAuthUserDataByUsernameStmt(cornucopia_async::private::Stmt::new(
                "SELECT id, username, password_hash
FROM users
WHERE username = $1",
            ))
        }
        pub struct GetAuthUserDataByUsernameStmt(
            cornucopia_async::private::Stmt,
        );
        impl GetAuthUserDataByUsernameStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                username: &'a T1,
            ) -> GetAuthUserDataByUsernameQuery<
                'a,
                C,
                GetAuthUserDataByUsername,
                1,
            > {
                GetAuthUserDataByUsernameQuery {
                    client,
                    params: [username],
                    stmt: &mut self.0,
                    extractor: |row| GetAuthUserDataByUsernameBorrowed {
                        id: row.get(0),
                        username: row.get(1),
                        password_hash: row.get(2),
                    },
                    mapper: |it| <GetAuthUserDataByUsername>::from(it),
                }
            }
        }
        pub fn get_auth_user_data_by_id() -> GetAuthUserDataByIdStmt {
            GetAuthUserDataByIdStmt(cornucopia_async::private::Stmt::new(
                "SELECT id, username, password_hash
FROM users
WHERE id = $1",
            ))
        }
        pub struct GetAuthUserDataByIdStmt(cornucopia_async::private::Stmt);
        impl GetAuthUserDataByIdStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                id: &'a i32,
            ) -> GetAuthUserDataByIdQuery<'a, C, GetAuthUserDataById, 1>
            {
                GetAuthUserDataByIdQuery {
                    client,
                    params: [id],
                    stmt: &mut self.0,
                    extractor: |row| GetAuthUserDataByIdBorrowed {
                        id: row.get(0),
                        username: row.get(1),
                        password_hash: row.get(2),
                    },
                    mapper: |it| <GetAuthUserDataById>::from(it),
                }
            }
        }
        pub fn check_if_user_exists_already() -> CheckIfUserExistsAlreadyStmt {
            CheckIfUserExistsAlreadyStmt(cornucopia_async::private::Stmt::new(
                "SELECT id FROM users
WHERE email = $1 OR username = $2",
            ))
        }
        pub struct CheckIfUserExistsAlreadyStmt(
            cornucopia_async::private::Stmt,
        );
        impl CheckIfUserExistsAlreadyStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                email: &'a T1,
                username: &'a T2,
            ) -> I32Query<'a, C, i32, 2> {
                I32Query {
                    client,
                    params: [email, username],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                CheckIfUserExistsAlreadyParams<T1, T2>,
                I32Query<'a, C, i32, 2>,
                C,
            > for CheckIfUserExistsAlreadyStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a CheckIfUserExistsAlreadyParams<T1, T2>,
            ) -> I32Query<'a, C, i32, 2> {
                self.bind(client, &params.email, &params.username)
            }
        }
        pub fn insert_new_user_settings() -> InsertNewUserSettingsStmt {
            InsertNewUserSettingsStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO user_settings DEFAULT VALUES returning id",
            ))
        }
        pub struct InsertNewUserSettingsStmt(cornucopia_async::private::Stmt);
        impl InsertNewUserSettingsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> I32Query<'a, C, i32, 0> {
                I32Query {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        pub fn insert_new_user() -> InsertNewUserStmt {
            InsertNewUserStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO users
(user_settings_id, username, bio, email, password_hash, status, role)
VALUES ($1, $2, NULL, $3, $4, NULL, $5) returning id",
            ))
        }
        pub struct InsertNewUserStmt(cornucopia_async::private::Stmt);
        impl InsertNewUserStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                user_settings_id: &'a i32,
                username: &'a T1,
                email: &'a T2,
                password_hash: &'a T3,
                role: &'a super::super::types::public::Userrole,
            ) -> I32Query<'a, C, i32, 5> {
                I32Query {
                    client,
                    params: [
                        user_settings_id,
                        username,
                        email,
                        password_hash,
                        role,
                    ],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertNewUserParams<T1, T2, T3>,
                I32Query<'a, C, i32, 5>,
                C,
            > for InsertNewUserStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertNewUserParams<T1, T2, T3>,
            ) -> I32Query<'a, C, i32, 5> {
                self.bind(
                    client,
                    &params.user_settings_id,
                    &params.username,
                    &params.email,
                    &params.password_hash,
                    &params.role,
                )
            }
        }
        pub fn insert_user_image() -> InsertUserImageStmt {
            InsertUserImageStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO objects
(key, object_type, avatar_users_id)
VALUES ($1, 'image', $2)",
            ))
        }
        pub struct InsertUserImageStmt(cornucopia_async::private::Stmt);
        impl InsertUserImageStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                key: &'a T1,
                users_id: &'a i32,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[key, users_id]).await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertUserImageParams<T1>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<
                                Output = Result<u64, tokio_postgres::Error>,
                            > + Send
                            + 'a,
                    >,
                >,
                C,
            > for InsertUserImageStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertUserImageParams<T1>,
            ) -> std::pin::Pin<
                Box<
                    dyn futures::Future<
                            Output = Result<u64, tokio_postgres::Error>,
                        > + Send
                        + 'a,
                >,
            > {
                Box::pin(self.bind(client, &params.key, &params.users_id))
            }
        }
    }
}
