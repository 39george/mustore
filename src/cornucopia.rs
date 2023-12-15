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
    pub mod tests {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUserCandidateByUsername {
            pub id: i32,
            pub created_at: time::OffsetDateTime,
            pub username: String,
            pub email: String,
            pub password_hash: String,
            pub validation_token: String,
        }
        pub struct GetUserCandidateByUsernameBorrowed<'a> {
            pub id: i32,
            pub created_at: time::OffsetDateTime,
            pub username: &'a str,
            pub email: &'a str,
            pub password_hash: &'a str,
            pub validation_token: &'a str,
        }
        impl<'a> From<GetUserCandidateByUsernameBorrowed<'a>>
            for GetUserCandidateByUsername
        {
            fn from(
                GetUserCandidateByUsernameBorrowed {
                    id,
                    created_at,
                    username,
                    email,
                    password_hash,
                    validation_token,
                }: GetUserCandidateByUsernameBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    created_at,
                    username: username.into(),
                    email: email.into(),
                    password_hash: password_hash.into(),
                    validation_token: validation_token.into(),
                }
            }
        }
        pub struct GetUserCandidateByUsernameQuery<
            'a,
            C: GenericClient,
            T,
            const N: usize,
        > {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor:
                fn(&tokio_postgres::Row) -> GetUserCandidateByUsernameBorrowed,
            mapper: fn(GetUserCandidateByUsernameBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetUserCandidateByUsernameQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetUserCandidateByUsernameBorrowed) -> R,
            ) -> GetUserCandidateByUsernameQuery<'a, C, R, N> {
                GetUserCandidateByUsernameQuery {
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
        pub fn get_user_candidate_by_username() -> GetUserCandidateByUsernameStmt
        {
            GetUserCandidateByUsernameStmt(
                cornucopia_async::private::Stmt::new(
                    "SELECT * FROM user_candidates
WHERE username = $1",
                ),
            )
        }
        pub struct GetUserCandidateByUsernameStmt(
            cornucopia_async::private::Stmt,
        );
        impl GetUserCandidateByUsernameStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                username: &'a T1,
            ) -> GetUserCandidateByUsernameQuery<
                'a,
                C,
                GetUserCandidateByUsername,
                1,
            > {
                GetUserCandidateByUsernameQuery {
                    client,
                    params: [username],
                    stmt: &mut self.0,
                    extractor: |row| GetUserCandidateByUsernameBorrowed {
                        id: row.get(0),
                        created_at: row.get(1),
                        username: row.get(2),
                        email: row.get(3),
                        password_hash: row.get(4),
                        validation_token: row.get(5),
                    },
                    mapper: |it| <GetUserCandidateByUsername>::from(it),
                }
            }
        }
    }
    pub mod user_auth_queries {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InsertUserToCandidatesParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
        > {
            pub username: T1,
            pub email: T2,
            pub password_hash: T3,
            pub validation_token: T4,
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
        pub fn insert_user_to_candidates() -> InsertUserToCandidatesStmt {
            InsertUserToCandidatesStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO user_candidates
    (username, email, password_hash, validation_token)
VALUES ($1, $2, $3, $4)",
            ))
        }
        pub struct InsertUserToCandidatesStmt(cornucopia_async::private::Stmt);
        impl InsertUserToCandidatesStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                username: &'a T1,
                email: &'a T2,
                password_hash: &'a T3,
                validation_token: &'a T4,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client
                    .execute(
                        stmt,
                        &[username, email, password_hash, validation_token],
                    )
                    .await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertUserToCandidatesParams<T1, T2, T3, T4>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<
                                Output = Result<u64, tokio_postgres::Error>,
                            > + Send
                            + 'a,
                    >,
                >,
                C,
            > for InsertUserToCandidatesStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertUserToCandidatesParams<T1, T2, T3, T4>,
            ) -> std::pin::Pin<
                Box<
                    dyn futures::Future<
                            Output = Result<u64, tokio_postgres::Error>,
                        > + Send
                        + 'a,
                >,
            > {
                Box::pin(self.bind(
                    client,
                    &params.username,
                    &params.email,
                    &params.password_hash,
                    &params.validation_token,
                ))
            }
        }
    }
}
