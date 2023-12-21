# MUSTORE API

- User signup:
```
→ TO
POST /api/signup
Form: {
    username: String,
    password: String,
    email: String,
    user_role: UserRole ('creator', 'consumer') OR admin_token: Uuid,
}
← FROM
If OK, StatusCode::Created (201)
if Err, 1. If internal error > StatusCode::InternalError (500)
        2. If bad input (for example if user already exists) > StatusCode::BadRequest (400) 
```
> When user signs up, a confirmation link is sent to the provided email address. This link is valid for 30 minutes. 

- User login:

- Get beats, songs, covers, lyrics counts:
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

