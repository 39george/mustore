# MUSTORE API

- User signup:
```
→ TO
POST /api/signup
Form: {
    username: String,
    password: String,
    email: String,
    user_role: UserRole ('creator', 'consumer'), OPTIONAL
    admin_token: Uuid, OPTIONAL
}
← FROM
If OK, StatusCode::Created (201)
if Err, 1. If internal error > StatusCode::InternalError (500)
        2. If bad input (for example if user already exists) > StatusCode::BadRequest (400) 
```
> When user signs up, a confirmation link is sent to the provided email address. This link is valid for 30 minutes. 

- User login:
```
→ TO
POST /api/login
Json: {
    username: String,
    password: String,
}
← FROM
If OK, StatusCode::OK (200)
if Err, 1. If internal error > StatusCode::InternalError (500)
        2. If bad input > StatusCode::Unautorized (401) 
```

- User logout:
```
→ TO
GET /api/logout
← FROM
If OK, browser will be redirected (we should make decision where to)
if Err > StatusCode::InternalError (500)
```

- Check user permissions:
```
→ TO
GET /api/protected/health_check
        OR /api/protected/admin/health_check
        OR /api/protected/creator/health_check
        OR /api/protected/consumer/health_check
← FROM
If OK > StatusCode::OK (200)
if Err > StatusCode::Forbidden (403)
```

- Get beats, songs, covers, lyrics COUNTS:
```
→ TO
GET /api/open/stats
← FROM
If OK, Json {
  songs: number,
  beats: number,
  covers: number,
  lyrics: number,
}, StatusCode::OK (200)
if Err > StatusCode::InternalError (500)
```

- Get genres or tags list:
```
→ TO
GET /api/open/genres OR GET /api/open/tags
← FROM
If OK, Json [..list], StatusCode::OK (200)
if Err, 1. If internal error > StatusCode::InternalError (500)
        2. If bad input > StatusCode::BadRequest (400) 
```

- Get songs objects:
```
→ TO
GET /api/open/songs
Json: {
    sex: Option<Sex>,
    tempo: Option<Vec<i16>>,
    key: Option<Vec<MusicKey>>,
    genres: Option<Vec<String>>,
    vibes: Option<Vec<String>>,
    sort_by: SortBy,
    amount: i64,
}
← FROM
If OK, Json [..list], StatusCode::OK (200)
if Err, 1. If internal error > StatusCode::InternalError (500)
        2. If bad input > StatusCode::BadRequest (400) 
```

- Get new songs:
```
→ TO
GET /api/open/new_songs?amount=<arg>
← FROM
If OK, Json [..list], StatusCode::OK (200)
if Err, 1. If internal error > StatusCode::InternalError (500)
        2. If bad input > StatusCode::BadRequest (400) 
```
- Get recommended songs:
```
→ TO
GET /api/open/recommended_songs?amount=<arg>
← FROM
If OK, Json [..list], StatusCode::OK (200)
if Err, 1. If internal error > StatusCode::InternalError (500)
        2. If bad input > StatusCode::BadRequest (400) 
```

