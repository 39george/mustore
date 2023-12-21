// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {}
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
        pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> &str,
            mapper: fn(&str) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> StringQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(&str) -> R,
            ) -> StringQuery<'a, C, R, N> {
                StringQuery {
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
        pub fn get_genres_list() -> GetGenresListStmt {
            GetGenresListStmt(cornucopia_async::private::Stmt::new(
                "SELECT name from genres",
            ))
        }
        pub struct GetGenresListStmt(cornucopia_async::private::Stmt);
        impl GetGenresListStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> StringQuery<'a, C, String, 0> {
                StringQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn get_tags_list() -> GetTagsListStmt {
            GetTagsListStmt(cornucopia_async::private::Stmt::new(
                "SELECT name from tags",
            ))
        }
        pub struct GetTagsListStmt(cornucopia_async::private::Stmt);
        impl GetTagsListStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> StringQuery<'a, C, String, 0> {
                StringQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
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
            pub id: i32,
            pub key: String,
            pub username: String,
            pub email: String,
        }
        pub struct SelectUserDataWithAvatarKeyBorrowed<'a> {
            pub id: i32,
            pub key: &'a str,
            pub username: &'a str,
            pub email: &'a str,
        }
        impl<'a> From<SelectUserDataWithAvatarKeyBorrowed<'a>>
            for SelectUserDataWithAvatarKey
        {
            fn from(
                SelectUserDataWithAvatarKeyBorrowed {
                    id,
                    key,
                    username,
                    email,
                }: SelectUserDataWithAvatarKeyBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    key: key.into(),
                    username: username.into(),
                    email: email.into(),
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
                    "SELECT users.id, objects.key, username, email
FROM users
JOIN objects
ON users.id = objects.avatar_users_id
WHERE users.username = $1",
                ),
            )
        }
        pub struct SelectUserDataWithAvatarKeyStmt(
            cornucopia_async::private::Stmt,
        );
        impl SelectUserDataWithAvatarKeyStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                username: &'a T1,
            ) -> SelectUserDataWithAvatarKeyQuery<
                'a,
                C,
                SelectUserDataWithAvatarKey,
                1,
            > {
                SelectUserDataWithAvatarKeyQuery {
                    client,
                    params: [username],
                    stmt: &mut self.0,
                    extractor: |row| SelectUserDataWithAvatarKeyBorrowed {
                        id: row.get(0),
                        key: row.get(1),
                        username: row.get(2),
                        email: row.get(3),
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
        }
        #[derive(Debug)]
        pub struct StoreUserPermissionParams<T1: cornucopia_async::StringSql> {
            pub user_id: i32,
            pub permission: T1,
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
        pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> &str,
            mapper: fn(&str) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> StringQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(&str) -> R,
            ) -> StringQuery<'a, C, R, N> {
                StringQuery {
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
        pub fn get_user_permissions() -> GetUserPermissionsStmt {
            GetUserPermissionsStmt(cornucopia_async::private::Stmt::new(
                "SELECT DISTINCT permissions.name
FROM users
JOIN users_groups
ON users.id = users_groups.users_id
JOIN groups_permissions
ON users_groups.groups_id = groups_permissions.groups_id
JOIN permissions
ON groups_permissions.permissions_id = permissions.id
WHERE users.id = $1",
            ))
        }
        pub struct GetUserPermissionsStmt(cornucopia_async::private::Stmt);
        impl GetUserPermissionsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                user_id: &'a i32,
            ) -> StringQuery<'a, C, String, 1> {
                StringQuery {
                    client,
                    params: [user_id],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
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
(user_settings_id, username, bio, email, password_hash, status)
VALUES ($1, $2, NULL, $3, $4, NULL) returning id",
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
            ) -> I32Query<'a, C, i32, 4> {
                I32Query {
                    client,
                    params: [user_settings_id, username, email, password_hash],
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
                I32Query<'a, C, i32, 4>,
                C,
            > for InsertNewUserStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertNewUserParams<T1, T2, T3>,
            ) -> I32Query<'a, C, i32, 4> {
                self.bind(
                    client,
                    &params.user_settings_id,
                    &params.username,
                    &params.email,
                    &params.password_hash,
                )
            }
        }
        pub fn store_user_permission() -> StoreUserPermissionStmt {
            StoreUserPermissionStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO users_groups (users_id, groups_id)
VALUES (
    $1,
    (SELECT id FROM groups WHERE name = $2)
)",
            ))
        }
        pub struct StoreUserPermissionStmt(cornucopia_async::private::Stmt);
        impl StoreUserPermissionStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                user_id: &'a i32,
                permission: &'a T1,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[user_id, permission]).await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                StoreUserPermissionParams<T1>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<
                                Output = Result<u64, tokio_postgres::Error>,
                            > + Send
                            + 'a,
                    >,
                >,
                C,
            > for StoreUserPermissionStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a StoreUserPermissionParams<T1>,
            ) -> std::pin::Pin<
                Box<
                    dyn futures::Future<
                            Output = Result<u64, tokio_postgres::Error>,
                        > + Send
                        + 'a,
                >,
            > {
                Box::pin(self.bind(client, &params.user_id, &params.permission))
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
